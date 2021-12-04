use std::io;
use std::io::Write;

fn read(line: String) -> String {
    line
}

fn eval(line: String) -> String {
    line
}

fn print(line: String) -> String {
    line
}

fn rep(mut line: String) -> String {
    line = read(line);
    line = eval(line);
    line = print(line);
    line
}

fn print_prompt() {
    print!("user> ");
    io::stdout().flush().unwrap();
}

fn main() {
    print_prompt();
    let mut line = String::new();
    while io::stdin().read_line(&mut line)
        .expect("Error reading line") > 0 {
        line = rep(line);
        println!("{}", line);
        print_prompt();
        }
    print!("\n");
    io::stdout().flush().unwrap();
}
