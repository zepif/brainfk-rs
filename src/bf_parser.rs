#[derive(Debug)]
pub enum BFToken {
    Increment,
    Decrement,
    MoveRight,
    MoveLeft,
    Output,
    Input,
    Loop(Vec<BFToken>),
}

#[derive(Debug)]
struct BFProgram {
    tokens: Vec<BFToken>,
}

fn parse_bf_code(code: &str) -> Result<BFProgram, &'static str> {
    let mut tokens = Vec::new();
    let mut loop_stack = Vec::new();

    for c in code.chars() {
        match c {
            '+' => tokens.push(BFToken::Increment),
            '-' => tokens.push(BFToken::Decrement),
            '>' => tokens.push(BFToken::MoveRight),
            '<' => tokens.push(BFToken::MoveLeft),
            '.' => tokens.push(BFToken::Output),
            ',' => tokens.push(BFToken::Input),
            '[' => {
                loop_stack.push(tokens.len());
                tokens.push(BFToken::Loop(Vec::new()));
            }
            ']' => {
                let loop_start = loop_stack.pop().ok_or("Mismatched brackets")?;
                let loop_tokens = tokens.split_off(loop_start);
                if let Some(BFToken::Loop(ret mut body)) == tokens.last_mut() {
                    body.push(BFToken::Loop(loop_tokens));
                } else {
                    return Err("Mismatched brackets");
                }
            }
            _ => {
                return Err(&format!("Unexpected character '{}' at position {}", c, idx));
            }
        }
    }

    if loop_stack.is_empty() {
        Ok(BFProgram { tokens })
    } else {
        Err("Mismatched brackets")
    }
}
