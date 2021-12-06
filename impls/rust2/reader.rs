use regex::Regex;
use crate::types::{MalType, MalList};

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

    fn next(&self) -> &String {
        match self.get_token() {
            Ok(v) => { self.pos += 1; &v},
            Err(e) => panic!("Panic: {}", e),
        }
    }

    fn peek(&self) -> &String {
        match self.get_token() {
            Ok(v) => &v,
            Err(e) => panic!("Panic: {}", e),
        }
    }

    fn get_token(&self) -> Result<String, String>{
        match self.tokens.get(self.pos) {
            Some(val) => Ok(val.to_string()),
            None => Err("pos out of bounds".to_string()),
        }
    }
}

pub fn read_str(string: &String) {
    let reader = Reader::new(tokenize(&string));
    read_form(reader);
}

fn tokenize(string: &str) -> Vec<String> {
    let mut ret = Vec::<String>::new();
    for token in Regex::new(
        r###"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"###
        ).unwrap().find_iter(string) {
            ret.push(token.as_str().to_string()); 
    }
    ret
}

fn read_form(reader: Reader) -> MalType {
    let chars = reader.peek().chars();
    match chars.next() {
        Some('(') => {
                read_list(reader)
        }
        _   => {
                read_atom(reader)
        }
    }
}

fn read_list(reader: Reader) -> MalType {
    let mut ret = MalType::MalList(MalList::new());
    loop {
        match chars.next() {
            Some(')') => break;
            _ => ret.push(read_atom(reader));
        }
    }
    let m_type = read_form(reader);
    while m_type = read_form(reader) {
        ret.push(read_form(reader));
    ret
    }
}

fn read_atom(reader: Reader) -> MalType {
}
