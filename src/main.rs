use std::{
    fs,
    io::{Result, prelude::*},
};

mod parser;
mod scanner;
mod syntax;
mod token;

fn run(source: &str) {
    let mut scanner = scanner::Scanner::new(source.to_string());
    scanner.scan_tokens();

    println!("{:#?}", &scanner.tokens);
}

fn run_script(path: &String) -> Result<()> {
    let script = fs::read_to_string(path)?;
    run(script.as_str());
    Ok(())
}

fn run_prompt() -> Result<()> {
    loop {
        print!("> ");

        let mut input = String::new();
        std::io::stdout().flush()?;
        std::io::stdin().read_line(&mut input)?;

        if input.trim() == "exit" {
            break;
        }

        run(input.as_str());
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        1 => run_prompt(),
        2 => run_script(&args[1]),
        _ => Ok(()),
    }
}
