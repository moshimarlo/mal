use regex::Regex;
use crate::types::{
    MalType,
    MalList,
    MalNumber,
    MalSymbol,
    MType,
};

struct Reader {
    tokens: Vec<String>,
    pos: usize,
}

impl Reader {
    fn new(v: Vec<String>) -> Reader {
        Reader {
            tokens: v,
            pos: 0,
        }
    }

    fn next(&mut self) -> String {
        let ret: String;
        match self.get_token() {
            Ok(v) => ret = v.to_string(),
            Err(e) => panic!("Panic: {}", e),
        }
        self.pos += 1;
        ret
    }

    fn peek(&self) -> &str {
        match self.get_token() {
            Ok(v) => &v,
            Err(e) => panic!("Panic: {}", e),
        }
    }

    fn get_token(&self) -> Result<&str, &str>{
        match self.tokens.get(self.pos) {
            Some(val) => Ok(val),
            None => Err("pos out of bounds"),
        }
    }
}

pub fn read_str(s: &str) -> MalType {
    let mut reader = Reader::new(tokenize(s));
    read_form(&mut reader)
}

fn tokenize(s: &str) -> Vec<String> {
    let mut ret = Vec::<String>::new();
    for token in Regex::new(
        r###"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"###
        ).unwrap().find_iter(s) {
            ret.push(token.as_str().to_string()); 
    }
    ret
}

fn read_form(reader: &mut Reader) -> MalType {
    let mut chars = reader.peek().chars();
    match chars.next() {
        Some('(') => {
            let val = read_list(reader);
            val
        },
        _ => {
            read_atom(&reader)
        }
    }
}

fn read_list(reader: &mut Reader) -> MalType {
    let mut ret = MalType::List(MalList::new());
    let mut s: String;
    loop {
        s = reader.next();
        match s.as_ref() {
            ")" => { break; },
            _ => {
                //ret.push::<MalList>(read_atom(&reader))
                //MalType(<MalList>::push(&ret, read_atom(&reader)));
                //MalType::List(MalList::push(&ret, read_atom(&reader)));
                Some(ret.push(read_form(reader)));
            },
        }
    }
    ret
}

fn read_atom(reader: &Reader) -> MalType {
    let tok = reader.peek();
    match tok.parse() {
        Ok(val) => MalType::Number(MalNumber::new(val)),
        Err(_) => MalType::Symbol(MalSymbol::new(&tok[..])),
    }
}
