use std::io;

pub fn input_handler(input: &mut String) -> Result<(), std::io::Error> {
    io::stdin().read_line(input)?;
    Ok(())
}
