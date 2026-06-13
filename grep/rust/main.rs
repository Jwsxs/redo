use std::{env, error::Error, fs::File, io::Read};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    
    let input_file = match args.get(1) {
        Some(f) => f,
        None => {
            eprintln!("Input file missing:\nCorrect usage: {}", "cargo run {file} {string}" );
            std::process::exit(1);
        }
    };

    let string: String = match args.get(2) {
        Some(str) => str.to_string(),
        None => {
            eprintln!("Input string missing:\nCorrect usage:{}", "cargo run {file} {string}");
            std::process::exit(1);
        }
    };

    let mut file: File = File::open(input_file)?;

    println!("Searching for {} out on {}\n\n===\n", string, input_file);

    let mut contents = String::new();
    file.read_to_string(&mut contents);
    
    for (i, line) in contents.lines().enumerate() {
        if line.contains(&string) {
            println!("{}: {}", i + 1, line);
        }
    }
/*
    println!("{}", args.display());
    Ok(())
*/
    Ok(())
}
