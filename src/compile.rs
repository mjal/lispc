use super::parse::Token;

#[derive(Debug,Clone)]
pub enum SExpr {
    Symbol(String),
    List(Vec<SExpr>)
}

pub fn compile(tokens : Vec<Token>) -> SExpr {
    return compile2(tokens.as_slice(), 0);
}

fn compile2(tokens : &[Token], scope : i32) -> SExpr {
    let mut list : Vec<SExpr> = Vec::new();
    let mut skip_until = 0;
    for (i, tok) in tokens.iter().enumerate() {
        if i < skip_until {
            continue;
        }
        match tok {
            Token::Word(string) => {
                if string != "\n" {
                    list.push(SExpr::Symbol(string.to_owned()))
                }
            },
            Token::ParenthesisOpen => {
                let n = find_closing_parenthesis(&tokens[(i+1)..]);
                list.push(compile2(&tokens[(i+1)..(i + 1 + n)], scope + 1));
                skip_until = i + 1 + n;
            },
            Token::ParenthesisClose => {},
        }
    }
    if list.len() == 1 {
        return list.remove(0); // Take the first element
    }
    return SExpr::List(list);
}

fn find_closing_parenthesis(tokens : &[Token]) -> usize {
    let mut count : i32 = 0;
    for (i, tok) in tokens.iter().enumerate() {
        match tok {
            Token::ParenthesisOpen => count += 1,
            Token::Word(_) => (),
            Token::ParenthesisClose => {
                if count == 0 {
                    return i;
                }
                count -= 1;
            }
        }
    }
    eprintln!("Unbalanced parenthesis");
    return 0;
}
