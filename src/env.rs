use crate::{parse_list_of_floats, LispErr, LispExp};
use std::collections::HashMap;

#[derive(Clone, Default)]
pub struct LispEnv {
    pub data: HashMap<String, LispExp>,
}

impl LispEnv {
    pub fn new() -> Self {
        let mut data = HashMap::new();
        data.insert(
            "+".to_string(),
            LispExp::Func(|args| {
                let sum = parse_list_of_floats(args)?
                    .iter()
                    .fold(0.0, |sum, a| sum + a);
                Ok(LispExp::Number(sum))
            }),
        );

        data.insert(
            "-".to_string(),
            LispExp::Func(|args| {
                let floats = parse_list_of_floats(args)?;
                let first = *floats
                    .first()
                    .ok_or_else(|| LispErr::Reason("expected at least one number".to_string()))?;
                let sum_of_rest = floats[1..].iter().fold(0.0, |sum, a| sum + a);
                Ok(LispExp::Number(first - sum_of_rest))
            }),
        );

        LispEnv { data }
    }
}
