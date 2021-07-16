use core::fmt;

use crate::LispErr;

#[derive(Clone)]
pub enum LispExp {
    Symbol(String),
    Number(f64),
    List(Vec<LispExp>),
    Func(fn(&[LispExp]) -> Result<LispExp, LispErr>),
}

impl fmt::Display for LispExp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = match self {
            LispExp::Symbol(s) => s.clone(),
            LispExp::Number(n) => n.to_string(),
            LispExp::List(list) => {
                let xs: Vec<String> = list.iter().map(|x| x.to_string()).collect();
                format!("({})", xs.join(","))
            }
            LispExp::Func(_) => "Function {}".to_string(),
        };

        write!(f, "{}", string)
    }
}
