use super::compile::SExpr;

pub fn print(sexpr : &SExpr) -> () {
    match sexpr {
        SExpr::Symbol(string) => print!("{}", string),
        SExpr::List(list) => {
            print!("(");
            for (i, el) in list.iter().enumerate() {
                if i != 0 {
                    print!(" ");
                }
                print(el);
            }
            print!(")");
        }
    }
}
