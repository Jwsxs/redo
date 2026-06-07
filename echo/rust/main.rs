use std::env;

fn main() {
    // args come formatted in a vector of string, since it has no size limited and can grow
    // we fetch it from environment, selecting our args and literally colleting them
    let args: Vec<String> = env::args().collect();

    // one way we check args were correctly fetched is by
    // dbg!(&args);
    // tho i'm not sure if it's really for print-debugging purposes, since it also returns some value

    // now we literally just print out the 2nd value of args
    // since the 1st is the file being run, 2nd and above are the real arguments
    println!("{}", args[1]);
}
