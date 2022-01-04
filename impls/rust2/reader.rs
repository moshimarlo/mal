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

    fn next(&mut self) -> &str {
        let s = match self.tokens.get(self.pos) {
            Some(s) => {
                s
            },
            None => "EOF",
        };
        self.pos += 1;
        s
    }

    fn peek(&self) -> &str {
        match self.tokens.get(self.pos) {
            Some(v) => &v,
            None => "EOF",
        }
    }
}

pub fn read_str(s: &str) -> Option<MalType> {
    let mut reader = Reader::new(tokenize(s));
    println!("{:?}", reader.tokens);
    read_form(&mut reader)
}

fn tokenize(s: &str) -> Vec<String> {
    let mut v = Vec::<String>::new();
    for token in Regex::new(
        r###"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"###
        ).unwrap().find_iter(s) {
            v.push(
                token
                .as_str()
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect()
            );
    }
    v
}

fn read_form(reader: &mut Reader) -> Option<MalType> {
    let mut chars = reader.peek().chars();
    match chars.next() {
        Some('(') => {
            let val = read_list(reader);
            Some(val)
        },
        _ => {
            if let Some(val) = read_atom(&reader) {
                Some(val)
            } else {
                None
            }
        }
    }
}

fn read_list(reader: &mut Reader) -> MalType {
    let mut ret = MalType::List(MalList::new());
    let mut s: &str;
    loop {
        s = reader.next();
        match s.as_ref() {
            ")" => { break; },
            _ => {
                match read_form(reader) {
                    Some(val) => { ret.push(val); },
                    None => { continue; },
                }
            },
        }
    }
    ret
}

fn read_atom(reader: &Reader) -> Option<MalType> {
    let tok = reader.peek();
    match tok.parse() {
        Ok(num) => Some(MalType::Number(MalNumber::new(num))),
        Err(_) => { 
            match tok {
                ")" => None,
                _ => {
                    Some(MalType::Symbol(MalSymbol::new(&tok[..])))
                },
            }
        },
    }
}
