mod token;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        1 => ferrilox::run_prompt(),
        2 => ferrilox::run_script(&args[1]),
        _ => Ok(()),
    }
}
