use crate::lexer;
use crate::token::Token;
use std::io;
const PROMPT: &str = "\n>> ";

pub fn start<R: io::BufRead, W: io::Write>(mut reader: R, mut writer: W) -> io::Result<()> {
    let mut cont = true;
    while cont {
        writer.write(PROMPT.as_bytes())?;
        writer.flush()?;
        let mut line = String::new();
        reader.read_line(&mut line)?;
        let mut lex = lexer::new(&line);
        cont = false;
        loop {
            let tok = lex.next_token();
            writer.write(format!("{:?} ", tok).as_bytes())?;
            writer.flush()?;
            if tok == Token::EOF {
                break;
            }
            cont = true;
        }
    }
    #[warn(unreachable_code)]
    Ok(())
}
