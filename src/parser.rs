use crate::token::*;
use crate::operation::*;

pub fn generate_program(tokens: &[Token]) -> Vec<Operation> {
    use self::TokenKind::*;

    // An vector including all operations of the parsed program.
    let mut operations = vec![];

    for token in tokens.iter() {
        // The argument for the current operation. By default, 1 is assumed, but optimizations
        // might combine multiple ones.
        let argument = 1;

        // The current offset.
        let offset = 0;

        let kind = match token.kind {
            Add => OperationKind::Add,
            Sub => OperationKind::Sub,
            Right => OperationKind::Right,
            Left => OperationKind::Left,
            Read => OperationKind::Read,
            Write => OperationKind::Write,
            BeginLoop => OperationKind::BeginLoop,
            EndLoop => OperationKind::EndLoop,
        };

        let operation = Operation {
            kind,
            offset,
            argument,
            start: token.start + 1,
            end: token.end + 1,
        };

        operations.push(operation);
    }

    operations
}
