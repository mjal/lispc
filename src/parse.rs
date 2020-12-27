#[derive(Debug)]
pub enum Token {
    ParenthesisOpen,
    ParenthesisClose,
    Word(String)
}

pub fn parse(content : String) -> Vec<Token> {
    let mut program : Vec<Token> = Vec::new();
    let mut word = String::new();

    for c in content.chars() {
        if word.len() != 0 {
            match c {
                '(' | ')' | ' ' => {
                    program.push(Token::Word(word));
                    word = String::new();
                },
                _ => ()
            }
        }
        match c {
            '(' => program.push(Token::ParenthesisOpen),
            ')' => program.push(Token::ParenthesisClose),
            ' ' => (),
            _   => word.push(c),
        }
    }

    if word.len() != 0 {
        program.push(Token::Word(word));
    }

    /*
    for tok in &program {
        match tok {
            Token::ParenthesisOpen  => println!("'('"),
            Token::ParenthesisClose => println!("')'"),
            Token::Word(s) => println!("{}", s),
        }
    }
    */

    return program;
}
