use monkey::repl::start;
use std::io;

fn main() -> io::Result<()> {
    let input = io::stdin();
    let output = io::stdout();
    start(input.lock(), output.lock())?;
    Ok(())
}
