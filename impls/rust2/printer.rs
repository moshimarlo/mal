use crate::types::{MalType, MalList};

pub fn pr_str(m: MalType) -> String {
    // match &m {
    //     //MalType::List(l) => { println!("Length: {}", l.len())},
    //     _ => {},
    // }
    String::from(m.to_string())
}