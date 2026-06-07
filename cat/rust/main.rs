use std::{fs::File, io::Read};
use std::env;

// doesn't have to return anything
fn file_read(file: &mut File) {
    let mut buf = [0u8; 1];

    // same idea as EOF on c= fgetc(FILE *file) being -1,
    // in this case it's 0, which means nothing is read
    while file.read(&mut buf).unwrap() > 0 {
        let byte = buf[0];
        print!("{}", byte as char);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    /*
     * Option<_>  = owns _
     * Option<&_> = borrows _
     */
    let input_file =  match args.get(1) {
        Some(path) => path, // something is existant, unpack and return it
        None => { // there's nothing, so we can call it an error
            eprintln!("Failed to parse path '{}'\n", args[1]);
            std::process::exit(1);
        }
    };

    let mut file = match File::open(input_file) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open file './{}'\nError: {}", input_file, e);
            std::process::exit(1);
        }
    };
/*
    // instead of C, we can just create a new String "buffer"
    let mut file_buffer = String::new();
    // and read the whole file content to it
    file.read_to_string(&mut file_buffer).unwrap();
    println!("Reading out './{}'\n\n===\n", input_file);
*/

    // then print out
    file_read(&mut file);
}
