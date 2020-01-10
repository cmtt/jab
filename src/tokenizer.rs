use super::token::*;

pub fn tokenize(input: &[u8]) -> Vec<Token> {
    let chars = input.iter();
    let mut tokens = Vec::<Token>::new();
    let mut offset = 0;

    use self::TokenKind::*;
    for c in chars {
        let kind = match c {
            b'+' => Add,
            b'-' => Sub,
            b'>' => Right,
            b'<' => Left,
            b',' => Read,
            b'.' => Write,
            b'[' => BeginLoop,
            b']' => EndLoop,
            _ => {
                offset += 1;
                continue;
            }
        };
        tokens.push(Token {
            kind,
            start: offset,
            end: offset + 1,
        });
        offset += 1;
    }
    tokens
}
