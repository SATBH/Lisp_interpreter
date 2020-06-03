
use regex::Regex;

#[derive(Debug)]
#[derive(Clone)]
pub enum Expr{
    Int(i64), Float(f64), Symbol(String), Sexpr(ParseTrie), Str(String), Empty, List(ParseTrie)
}

#[derive(Debug)]
#[derive(Clone)]
pub struct ParseTrie{
    pub childs: Vec<Expr>,
}

pub fn parse_expr( string: &str ) -> Result<Expr, String>{
    let integer = Regex::new("^\\d+$").unwrap();
    let float = Regex::new("^\\d+.\\d+$").unwrap();
    let sexpr = Regex::new("^\\([\\s\\S]*\\)$").unwrap();
    let string_type = Regex::new("^\"[\\s\\S]*\"").unwrap();
    
    if integer.is_match(string.trim()){
        return Ok(Expr::Int(string.trim().parse().unwrap()))
    }
    if float.is_match(string.trim()){
        return Ok(Expr::Float(string.trim().parse().unwrap()))
    }
    if sexpr.is_match(string){
        let s:String = string.chars().skip(1).take(string.len()-2).collect();
        return Ok(Expr::Sexpr(parse_string(&s).unwrap()))
    }
    if string_type.is_match(string){
        let s:String = string.chars().skip(1)
                       .take(string.len()-2).collect();
        return Ok(Expr::Str(string.to_string()));
    }
    return Ok(Expr::Symbol(string.trim().to_string()));
    Err("Invalid expression".to_string())
}

pub fn parse_string( string: &str ) -> Result<ParseTrie, String>{
    let mut trie: ParseTrie = ParseTrie{childs : Vec::new()};
    for s in split(string).into_iter(){
        match parse_expr(&s){
            Ok(t) => trie.childs.push(t),
            Err(t) => return Err(t)
        }
    }

    Ok(trie)
}
fn split(string: &str) -> Vec<String>{
	let mut n = 0;
	let mut ret = Vec::new();
	let mut buffer: String = String::new();
    let mut is_string: bool = false;
	for c in string.chars(){
		if n == 0 && c == ' ' && !is_string && buffer != ""{
			ret.push(buffer);
			buffer = String::new();
		}
        else{
			match c
            {
				'('  =>if !is_string {n += 1},
				')'  =>if !is_string {n -= 1},
                '"' => is_string = !is_string,
				 _   => ()
			}
			buffer.push(c)
		}
	}
	if buffer != ""{
	    ret.push(buffer.trim().to_string());
	}
	ret
}

