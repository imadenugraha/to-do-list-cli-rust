use std::io;

pub fn input_handler(input: &mut String) -> usize {
    io::stdin().read_line(input).expect("Can't read input")
}
