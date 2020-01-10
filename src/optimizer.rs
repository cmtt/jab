use std::collections::HashMap;
use std::cmp::Ordering;
use crate::operation::*;

pub fn optimize(operations: Vec<Operation>) -> Vec<Operation> {
    let operations = optimize_repetitions(operations);
    let operations = optimize_loops(operations);
    optimize_adjacent_operations(operations)
}

fn optimize_repetitions(operations: Vec<Operation>) -> Vec<Operation> {
    // An vector returning the optimized operations.
    let mut new_operations = vec![];

    use OperationKind::*;

    // These operations are repeatable.
    let repeat_ops: Vec<OperationKind> = vec![Add, Sub, Left, Right, Write, SetZero];

    let mut iter = operations.into_iter().peekable();

    loop {
        let current = iter.next();

        if current.is_none() {
            break;
        }

        let op = current.unwrap();
        let kind = op.kind;

        if !repeat_ops.contains(&kind) {
            new_operations.push(op);
            continue;
        }

        let offset = op.offset;
        let start = op.start;
        let mut end = op.end;
        let mut argument = op.argument;

        let mut next = iter.peek();

        loop {
            if next.is_none() {
                break
            }

            let next_op = next.unwrap();

            if next_op.kind == kind && next_op.argument == op.argument && next_op.offset == offset {
                argument += next_op.argument;
                end = next_op.end;
                iter.next();
                next = iter.peek();
            } else {
                break;
            }
        }

        new_operations.push(Operation {
            kind,
            argument,
            offset,
            start,
            end
        });
    }

    new_operations
}

fn optimize_loops(operations: Vec<Operation>) -> Vec<Operation> {
    // An vector returning the optimized operations.
    let mut new_operations: Vec<Operation> = vec![];

    use OperationKind::*;

    let mut iter = operations.into_iter();

    loop {
        let current = iter.next();

        if current.is_none() {
            break;
        }

        let op = current.unwrap();
        let len = new_operations.len();

        if len > 1 && op.kind == EndLoop && op.offset == 0 && op.argument == 1 {
            let first_op = new_operations[len - 2];
            let second_op = new_operations[len - 1];

            if first_op.kind == BeginLoop && (second_op.kind == Add || second_op.kind == Sub) {
                new_operations.pop();
                new_operations.pop();
                new_operations.push(Operation {
                    kind: SetZero,
                    argument: 1,
                    offset: 0,
                    start: first_op.start,
                    end: op.end,
                });
                continue;
            }
        }

        new_operations.push(op);
    }

    new_operations
}


pub fn optimize_adjacent_operations(program: Vec<Operation>) -> Vec<Operation> {
    // """Cancels out adjacent Add, Sub and Left Right.

    // E.g., ++++-->>+-<<< is equivalent to +<.
    // """
    let mut opposites = HashMap::new();
    opposites.insert(OperationKind::Add, OperationKind::Sub);
    opposites.insert(OperationKind::Sub, OperationKind::Add);
    opposites.insert(OperationKind::Left, OperationKind::Right);
    opposites.insert(OperationKind::Right, OperationKind::Left);

    let mut optimized = vec![];

    for op in program {
        let l = optimized.len();

        if l == 0 {
            optimized.push(op);
            continue;
        }

        let prev = optimized[l - 1];

        // Cancels out adjacent Add, Sub and Left Right
        // @see bfoptimization
        if Some(&prev.kind) == opposites.get(&op.kind) && prev.offset == op.offset {
            let x = (prev.argument as i32) - (op.argument as i32);
            match (x).cmp(&0) {
                // Negative: Use opposite operation.
                Ordering::Less => {
                    optimized[l - 1] = Operation {
                        kind: op.kind,
                        argument: (-x as u32),
                        offset: prev.offset,
                        start: prev.start,
                        end: op.end,
                    };
                },
                Ordering::Greater => {
                    optimized[l - 1] = Operation {
                        kind: prev.kind,
                        argument: x as u32,
                        offset: prev.offset,
                        start: prev.start,
                        end: op.end,
                    };
                },
                // Op cancelled out
                Ordering::Equal => {
                    optimized.remove(l - 1);
                }
            }
            continue;
        }

        optimized.push(op);
    }

    optimized
}


#[test]
fn test_optimize_repetitions() {
    use OperationKind::*;

    use crate::parser::generate_program;
    use crate::tokenizer::tokenize;
    let operations = generate_program(&tokenize(b"+++--<<<>>>"));
    let new_operations = optimize_repetitions(operations);

    assert!(new_operations == vec![
        Operation {
            kind: Add,
            argument: 3,
            offset: 0,
            start: 1,
            end: 4
        },
        Operation {
            kind: Sub,
            argument: 2,
            offset: 0,
            start: 4,
            end: 6
        },
        Operation {
            kind: Left,
            argument: 3,
            offset: 0,
            start: 6,
            end: 9
        },
        Operation {
            kind: Right,
            argument: 3,
            offset: 0,
            start: 9,
            end: 12
        },
    ]);
}


#[test]
fn test_optimize_loops() {
    use OperationKind::*;

    use crate::parser::generate_program;
    use crate::tokenizer::tokenize;

    let operations = generate_program(&tokenize(b"[-][+]"));
    let new_operations = optimize_loops(operations);

    assert!(new_operations == vec![
        Operation {
            kind: SetZero,
            argument: 1,
            offset: 0,
            start: 1,
            end: 4
        },
        Operation {
            kind: SetZero,
            argument: 1,
            offset: 0,
            start: 4,
            end: 7
        },
    ]);

    assert!(optimize_repetitions(new_operations) == vec![
        Operation {
            kind: SetZero,
            argument: 2,
            offset: 0,
            start: 1,
            end: 7
        }
    ]);
}