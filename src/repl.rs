use crate::lexer;
use crate::token::Token;
use std::io;
const PROMPT: &str = "\n>> ";

pub fn start<R: io::BufRead, W: io::Write>(mut reader: R, mut writer: W) -> io::Result<()> {
    loop {
        writer.write(PROMPT.as_bytes())?;
        writer.flush()?;
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Err(_) => break,
            Ok(_) => (),
        }
        let mut lex = lexer::new(&line);
        loop {
            let tok = lex.next_token();
            writer.write(format!("{:?} ", tok).as_bytes())?;
            writer.flush()?;
            if tok == Token::EOF {
                break;
            }
        }
    }
    Ok(())
}
