use lisp_rs::{eval, parse, tokenize, LispEnv, LispErr, LispExp};
use std::io;
use std::io::Write;

fn parse_eval(expr: String, env: &mut LispEnv) -> Result<LispExp, LispErr> {
    let (parsed_exp, _) = parse(&tokenize(expr))?;
    let evaled_exp = eval(&parsed_exp, env)?;

    Ok(evaled_exp)
}

fn slurp_expr() -> String {
    let mut expr = String::new();

    io::stdin()
        .read_line(&mut expr)
        .expect("Failed to read line");

    expr
}

fn main() {
    let env = &mut LispEnv::new();
    loop {
        print!("lisp > ");
        io::stdout().flush().unwrap();
        let expr = slurp_expr();
        match parse_eval(expr, env) {
            Ok(res) => println!("Result => {}", res),
            Err(e) => match e {
                LispErr::Reason(msg) => println!("Error => {}", msg),
            },
        }
    }
}
