use crate::{LispEnv, LispErr, LispExp};

pub fn eval(exp: &LispExp, env: &mut LispEnv) -> Result<LispExp, LispErr> {
    match exp {
        LispExp::Symbol(k) => env
            .data
            .get(k)
            .ok_or_else(|| LispErr::Reason(format!("unexpected symbol: `{}`", k)))
            .map(|x| x.clone()),
        LispExp::Number(_) => Ok(exp.clone()),
        LispExp::List(list) => {
            let first_form = list
                .first()
                .ok_or_else(|| LispErr::Reason("expected a non-empty list".to_string()))?;
            let arg_forms = &list[1..];
            let first_eval = eval(first_form, env)?;
            match first_eval {
                LispExp::Func(f) => {
                    let args_eval = arg_forms
                        .iter()
                        .map(|x| eval(x, env))
                        .collect::<Result<Vec<LispExp>, LispErr>>();
                    f(&args_eval?)
                }
                _ => Err(LispErr::Reason("first form must be a function".to_string())),
            }
        }
        LispExp::Func(_) => Err(LispErr::Reason("unexpected form".to_string())),
    }
}
