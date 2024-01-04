/// Errors for the program
#[derive(Debug)]
enum Error {
    PathIsRequired,
}


fn main() -> Result<(), Error> {
    let path = match std::env::args().nth(2) {
        Some(p) => p,
        None => return Err(Error::PathIsRequired),
    };
        

    dbg!(path);

    Ok(())
}
