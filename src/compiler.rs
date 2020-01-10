use dynasm::dynasm;
use dynasmrt::{DynasmApi, DynasmLabelApi};

use std::mem;
use std::u8;
use crate::operation::*;

const TAPE_SIZE: usize = 30000;

pub struct State {
    tape: [u8; TAPE_SIZE],
}

pub struct Program {
    code: dynasmrt::ExecutableBuffer,
    start: dynasmrt::AssemblyOffset,
}

dynasm!(ops
    ; .arch x64
    ; .alias a_current, r13
    ; .alias retval, rax
);

macro_rules! prologue {
    ($ops:ident) => {{
        let start = $ops.offset();
        dynasm!($ops
            ; sub rsp, 0x28
            ; mov [rsp + 0x30], rcx
            ; mov [rsp + 0x40], r8
            ; mov [rsp + 0x48], r9
        );
        start
    }};
}

macro_rules! epilogue {
    ($ops:ident, $e:expr) => {dynasm!($ops
        ; mov retval, $e
        ; add rsp, 0x28
        ; ret
    );};
}

impl<'a> State {
    pub fn new() -> State {
        State {
            tape: [0; TAPE_SIZE],
        }
    }
}

impl Program {
    pub fn run(self, state: &mut State) -> Result<(), &'static str> {
        let f: extern "win64" fn(*mut State, *mut u8, *mut u8, *const u8) -> u8 = unsafe { mem::transmute(self.code.ptr(self.start)) };
        let start = state.tape.as_mut_ptr();
        let end =  unsafe { start.add(TAPE_SIZE) };
        let res = f(state, start, start, end);

        if res == 0 {
            Ok(())
        } else {
            panic!("Unknown error code");
        }
    }
}

pub fn compile (operations: &[Operation]) -> Result<Program, &'static str>  {
    let iter = operations.iter();

    let mut ops = dynasmrt::x64::Assembler::new().unwrap();
    let mut loops = Vec::new();

    let start = prologue!(ops);

    for op in iter {
        let kind = op.kind;
        let offset = op.offset;
        let argument = op.argument;

        let amount = argument as usize;

        use self::OperationKind::*;

        match kind {
            BeginLoop => {
                let backward_label = ops.new_dynamic_label();
                let forward_label = ops.new_dynamic_label();
                loops.push((backward_label, forward_label));
                dynasm!(ops
                    ; cmp BYTE [a_current + offset], 0
                    ; jz =>forward_label
                    ;=>backward_label
                );
            }

            EndLoop => {
                if let Some((backward_label, forward_label)) = loops.pop() {
                    dynasm!(ops
                        ; cmp BYTE [a_current + offset], 0
                        ; jnz =>backward_label
                        ;=>forward_label
                    );
                } else {
                    return Err("] without matching [");
                }
            }

            Add => {
                if amount == 1 {
                    dynasm!(ops
                        ; inc BYTE [a_current + offset]
                    );
                } else {
                    dynasm!(ops
                        ; add BYTE [a_current + offset], amount as _
                    );
                }
            }

            Sub => {
                if amount == 1 {
                    dynasm!(ops
                        ; dec BYTE [a_current + offset]
                    );
                } else {
                    dynasm!(ops
                        ; sub BYTE [a_current + offset], amount as _
                    );
                }
            }

            Right => {
                dynasm!(ops
                    ; add a_current, (amount % TAPE_SIZE) as _
                );
                if amount > TAPE_SIZE {
                    dynasm!(ops
                        ; cmp a_current, 0
                        ; jb >wrap
                        ; sub a_current, TAPE_SIZE as _
                        ;wrap:
                    );
                }
            }

            Left => {
                dynasm!(ops
                    ; sub a_current, (amount % TAPE_SIZE) as _
                );
                if amount > TAPE_SIZE {
                    dynasm!(ops
                        ; cmp a_current, 0
                        ; jae >wrap
                        ; add a_current, TAPE_SIZE as _
                        ;wrap:
                    );
                }
            }

            Read => {
                dynasm!(ops
                    ; mov rax, 0
                    ; mov rdi, 0
                    ; mov rsi, a_current
                );
                if offset != 0 {
                    if offset > 0 {
                        dynasm!(ops
                            ; add rsi, offset
                        );
                    } else {
                        dynasm!(ops
                            ; sub rsi, -offset
                        );
                    }
                }
                dynasm!(ops
                    ; mov rdx, 1
                    ; syscall
                );
            }

            Write => {
                dynasm!(ops
                    ; mov rax, 1
                    ; mov rdi, 1
                    ; mov rsi, a_current
                );
                if offset != 0 {
                    if offset > 0 {
                        dynasm!(ops
                            ; add rsi, offset
                        );
                    } else {
                        dynasm!(ops
                            ; sub rsi, -offset
                        );
                    }
                }
                for _ in 0..amount {
                    dynasm!(ops
                        ; mov rdx, 1
                        ; syscall
                    );
                }
            }

            SetZero => {
                dynasm!(ops
                    ; mov BYTE [a_current + offset], 0
                );
            }
        }
    }

    if !loops.is_empty() {
        return Err("[ without matching ]");
    }

    dynasm!(ops
        ;; epilogue!(ops, 0)
    );

    let code = ops.finalize().unwrap();

    Ok(Program {
        code,
        start,
    })
}