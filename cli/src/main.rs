use structopt::StructOpt;
use std::path::PathBuf;
use std::fs;
use base;
use ebnf;
use std::io::Write;

#[derive(Debug, StructOpt)]
pub struct Config {
    /// Grammar file path
    #[structopt(name = "GRAMMAR FILE", parse(from_os_str))]
    pub grammar_path: PathBuf,
    /// Initial production rule
    #[structopt(name = "INITIAL RULE")]
    pub initial_rule: String,
    /// Test string file path
    #[structopt(short = "t", long = "test", name = "TEST STRING FILE", parse(from_os_str))]
    pub test_string_path: Option<PathBuf>,
}

pub fn read() -> String {
    let mut buffer = String::new();
    print!("> ");
    std::io::stdout()
        .flush()
        .and(std::io::stdin().read_line(&mut buffer)).unwrap();
    return buffer;
}

fn main() {
    let config = Config::from_args();

    println!("parser-parser 0.1.0\n");

    let grammar = fs::read_to_string(config.grammar_path).unwrap();
    let grammar = match ebnf::parse(&grammar) {
        Ok(g) => {
            println!("Successfully parsed the provided grammar\n");
            g
        },
        Err(e) => {
            println!("Error: {} at position {}:{}", e, e.span.from.line, e.span.from.column);
            return;
        }
    };

    loop {
        let input = read();
        let input = input[..input.len() - 1].to_owned();
        match base::check(&input, &grammar, &config.initial_rule) {
            Some(_) => {
                println!("true")
            }
            None => {
                println!("false")
            },
        }
    }
}
