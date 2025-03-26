use std::{
    fs,
    io::{Result, prelude::*},
};

fn run(source: String) {
    print!("{source}");
}

fn run_script(path: &str) -> Result<()> {
    let script = fs::read_to_string(path)?;
    run(script);
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

        run(input);
    }

    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        1 => run_prompt(),
        2 => run_script(args[1].as_str()),
        _ => Ok(()),
    }
}
