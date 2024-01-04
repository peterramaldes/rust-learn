// I think this is the best way to return an error in command line 
// I don't know why some people use `fn main() -> Result<(), Error>`.
    
fn main() {
    let path = std::env::args().nth(2).ok_or_else(|| {
        eprintln!("ERROR: you must pass the path directory");
        std::process::exit(1);
    });
}
