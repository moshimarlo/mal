mod reader;
mod types;
mod printer;

use std::io::Write;

use types::{
    MalType,
};

fn read(line: String) -> MalType {
    if let Some(val) = reader::read_str(&line) {
        val
    } else {
        panic!("Error reading string");
    }
}

fn eval(line: MalType) -> MalType {
    line
}

fn print(line: MalType) -> String {
    printer::pr_str(line)
}

fn rep(line: String) -> String {
    let mut mtype = read(line);
    mtype = eval(mtype);
    print(mtype)
}

fn print_prompt() {
    print!("user> ");
    std::io::stdout().flush().unwrap();
}

fn main() {
    print_prompt();
    let mut line = String::new();
    while std::io::stdin().read_line(&mut line)
        .expect("Error reading line") > 0 {
        line = rep(line);
        println!("{}", line);
        print_prompt();
        line = String::new();
        }
    print!("\n");
    std::io::stdout().flush().unwrap();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rep() {
        let line = rep("123".to_string());
        assert_eq!(line, "123");
    }

    #[test]
    fn test_num_spaces() {
        let line = rep("123 ".to_string());
        assert_eq!(line, "123");
    }

    #[test]
    fn test_sym() {
        let line = rep("abc".to_string());
        assert_eq!(line, "abc");
    }

    #[test]
    fn test_sym_spaces() {
        let line = rep("abc ".to_string());
        assert_eq!(line, "abc");
    }

    #[test]
    fn test_list() {
        let line = rep("(123 456)".to_string());
        assert_eq!(line, "(123 456)");
    }
}