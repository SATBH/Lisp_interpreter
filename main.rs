extern crate regex;
use std::io::Write;
use regex::Regex;
use std::collections::HashMap;
mod eval;
mod parse;
fn main() {
    let mut input = String::new();
    let mut Global_env: builtin_functions::Environment = builtin_functions::Environment{contents: HashMap::new()};
    print!("\nlisp>");
    while let Ok(n) = std::io::stdin().read_line(&mut input){
        match input.trim(){
            "" => print!("\nlisp>"),
            "exit" => break,
            t @ _ => print!("{:?}\nlisp> ", eval::eval(&mut Global_env, &parse::parse_string(t).unwrap()))
        }
        std::io::stdout().flush().unwrap();
        input.clear();
        //print!("lisp> ")
    }

}

mod builtin_functions{
    use crate::parse::{ParseTrie, Expr};
    use crate::parse::Expr::*;
    use crate::eval::{eval, eval_expr};
    use std::collections::HashMap;
    
    pub struct Environment{
        pub contents: HashMap <String, Expr>
    }

    /*Implentation of builtin addition*/
    pub fn add(env:&mut Environment, first: &Expr, second: &Expr) -> Result<Expr, String> {
        Ok(match (first, second) {
            (Float(t),   Int(a)) | (Int(a), Float(t)) => Float(*t + *a as f64),
            (Float(t), Float(a)) => Float(*t + *a),
            (Int(a),     Int(b)) => Int(*a + *b),
            (Str(s1),   Str(s2)) => Str(String::new() + s1 + s2),
            (a @ _, List(t)) => {
                let buff = &do_over_list(env, &t.childs[0..], add)?;
                add(env, a, &buff)?
            },
            (Empty, t) | (t, Empty) => t.clone(),
            (_, _) => return Err("Invalid sum".to_string())
        })
    }
    /*subtraction implementation*/
    pub fn subtraction(env:&mut Environment, first :&Expr, second: &Expr) -> Result<Expr, String>{
        match (first, second) {
            (some, Int(a))   => add(env, some, &Int(- *a)),
            (some, Float(a)) => add(env, some, &Float(- *a)),
            (some, List(t))  => {
                let buffer = do_over_list(env, &t.childs[0..], add)?;
                subtraction(env, first, &buffer)
            },
            (Empty, some) => Ok(some.clone()),
            _ => Err(String::from("Invalid subtraction"))
        }
    }
    pub fn do_over_list(env:&mut Environment, values: &[Expr], fun: fn (&mut Environment, &Expr, &Expr) -> Result<Expr, String> ) -> Result<Expr, String>{
        let mut sum: Expr = Empty;
		    for i in values{
    		    let val = eval_expr(env, i)?;
		        sum = fun(env, &sum, &val)?;
		    }
		return Ok(sum)
    }
    pub fn define(env:&mut Environment, values: &[Expr]) -> Result<Expr, String>{
        if values.len() != 2 { return Err("Invalid number of arguments in define function".to_string()) }
        if let Symbol(t) = &values[0]{
            let buffer = eval_expr(env, &values[1])?;
            env.contents.insert(t.to_string(), buffer);
            return Ok(Empty);
        }
        else if let (Sexpr(a), List(b)) = (values[0].clone(), eval_expr(env, &values[1])?){
            if a.childs.len() == b.childs.len(){
                for num in 0..a.childs.len(){
                    define(env, &[a.childs[num].clone(), b.childs[num].clone()])?;
                }
                return Ok(Empty)
            }
            return Err("Expressions could not be matched".to_string());
        }
        Err("Invalid symbols passed as first argument".to_string())
    }
}
