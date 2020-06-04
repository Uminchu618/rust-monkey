use crate::lexer::Lexer;
use std::io;
use crate::parser::Parser;

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
        let mut lex = Lexer::new(&line);
        let mut parser = Parser::new(&mut lex);
        let program = parser.parse_program();
        for statement in program {
            writer.write(format!("{:?} ", statement).as_bytes())?;
            writer.flush()?;
        }
    }
    Ok(())
}
