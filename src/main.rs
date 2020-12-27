use std::env;
use std::fs;
mod parse;
mod compile;
mod exec;
mod print;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Not enough arguments");
        return;
    }

    let filename = &args[1];

    let content = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    print!("{}", content);

    let tokens = parse::parse(content);
    //println!("{:?}", tokens);
    let sexpr  = compile::compile(tokens);
    //println!("{:?}", sexpr);
    let executed_sexpr  = exec::exec(sexpr);
    //println!("{:?}", executed_sexpr);

    print::print(&executed_sexpr);
    print!("\n");
}
