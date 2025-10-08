use datetime::{interpreter::Interpreter, lexer::Token};
use miette::Error;

fn main() -> Result<(), Error> {
    let mut input = String::from("04-02?2003");
    dbg!(Interpreter::parse_datetime(
        &mut input,
        String::from("%d-%m-%Y")
    )?);
    Ok(())
}
