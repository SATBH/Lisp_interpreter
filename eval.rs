use crate::parse::{ParseTrie, Expr};
use crate::parse::Expr::*;
use crate::builtin_functions::{add, subtraction, do_over_list, define, Environment};

pub fn eval(env:&mut Environment, trie: &ParseTrie) -> Result<Expr, String>{
    if let Expr::Symbol(s) =  &trie.childs[0] {
        if s == "+"{
            return do_over_list(env, &trie.childs[1..], add)
        } else if s == "-"{
            return do_over_list(env, &trie.childs[1..], subtraction)
        }else if s == "define"{
            return define(env, &trie.childs[1..])
        }    
    }
    
    if trie.childs.len() == 1{
        eval_expr(env, &trie.childs[0])
    }
    else{
    	let mut list = ParseTrie{childs: Vec::new()};
    	for i in &trie.childs[0..]{
		    list.childs.push(eval_expr(env, i)?);
        }
        Ok(List(list))
    }
}

pub fn eval_expr(env:&mut Environment, val: &Expr) -> Result<Expr, String>{
    Ok(match val{
        Symbol(t) => match env.contents.get(t){
            Some(x) => match x.clone(){
                Sexpr(t) => eval(env, &t)?,
                t @ _ => t.clone()
            }
            None => Symbol(t.to_string())
        },
        Sexpr(t) => eval(env, &t)?,
        t @ _ =>t.clone() 
    })
}
