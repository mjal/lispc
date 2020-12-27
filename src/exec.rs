use super::compile::SExpr;

fn compute(list : Vec<SExpr>) -> SExpr {
    if list.len() == 0 {
        return SExpr::List(vec!());
    }
    match &list[0] {
        SExpr::Symbol(symbol) => match symbol.as_str() {
            "+" => return plus(&list[1..]),
            "-" => return minus(&list[1..]),
            _ => return SExpr::List(list),
        },
        _ => {
            return SExpr::List(list);
        }
    }
}

pub fn exec(sexpr : SExpr) -> SExpr {
    match sexpr {
        SExpr::Symbol(string) => return SExpr::Symbol(string),
        SExpr::List(vec) => {
            let simplified_sexpr : Vec<SExpr> = vec.iter().cloned().map(|el| exec(el) ).collect();
            return compute(simplified_sexpr);
        }
    }
}

fn plus(args : &[SExpr]) -> SExpr {
    let mut acc : f64 = 0.0;
    for arg in args {
        match arg {
            SExpr::List(_) => eprintln!("'+': Does not work with a lists"),
            SExpr::Symbol(string) => {
                let parsed = string.parse::<f64>();
                match parsed {
                    Ok(n) => acc += n,
                    Err(_) => eprintln!("'+': {} is not a number", string),
                }
            }
        }
    }
    return SExpr::Symbol(acc.to_string());
}

fn minus(args : &[SExpr]) -> SExpr {
    let mut acc : f64 = 0.0;
    for (i, arg) in args.iter().enumerate() {
        match arg {
            SExpr::List(_) => eprintln!("'+': Does not work with a lists"),
            SExpr::Symbol(string) => {
                let parsed = string.parse::<f64>();
                match parsed {
                    Ok(n) => if i == 0 { acc = n } else { acc -= n },
                    Err(_) => eprintln!("'+': {} is not a number", string),
                }
            }
        }
    }
    return SExpr::Symbol(acc.to_string());
}
