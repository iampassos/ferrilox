use std::{
    fs,
    io::{Result, prelude::*},
};

pub fn run(source: &str) {
    print!("{source}");
}

pub fn run_script(path: &String) -> Result<()> {
    let script = fs::read_to_string(path)?;
    run(script.as_str());
    Ok(())
}

pub fn run_prompt() -> Result<()> {
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

pub fn error(line: u32, message: &str) {
    println!("Error in line {line}:\n {message}");
}
