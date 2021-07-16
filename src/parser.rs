use crate::{LispErr, LispExp};

pub fn parse(tokens: &[String]) -> Result<(LispExp, &[String]), LispErr> {
    let (token, rest) = tokens
        .split_first()
        .ok_or_else(|| LispErr::Reason("could not get token".to_string()))?;
    match &token[..] {
        "(" => read_seq(rest),
        ")" => Err(LispErr::Reason("unexpected `)`".to_string())),
        _ => Ok((parse_atom(token), rest)),
    }
}

fn read_seq(tokens: &[String]) -> Result<(LispExp, &[String]), LispErr> {
    let mut res = vec![];
    let mut xs = tokens;
    loop {
        let (next_token, rest) = xs
            .split_first()
            .ok_or_else(|| LispErr::Reason("cloud not find closing `)`".to_string()))?;
        if next_token == ")" {
            return Ok((LispExp::List(res), rest));
        }
        let (exp, new_xs) = parse(&xs)?;
        res.push(exp);
        xs = new_xs;
    }
}

fn parse_atom(token: &str) -> LispExp {
    let potential_float = token.parse();
    match potential_float {
        Ok(v) => LispExp::Number(v),
        Err(_) => LispExp::Symbol(token.to_string()),
    }
}

pub fn parse_list_of_floats(args: &[LispExp]) -> Result<Vec<f64>, LispErr> {
    args.iter().map(|x| parse_single_float(x)).collect()
}

pub fn parse_single_float(exp: &LispExp) -> Result<f64, LispErr> {
    match exp {
        LispExp::Number(num) => Ok(*num),
        _ => Err(LispErr::Reason("expected a number".to_string())),
    }
}
