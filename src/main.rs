mod interpreter;
mod token;

use self::interpreter::BrainfuckInterpreter;
use self::token::BrainfuckToken;
use std::env;

fn main() {
    let arguments = env::args().collect::<Vec<String>>();

    if arguments.len() == 1 {
        println!("Usage: {} [brainfuck-program]", arguments[0]);
    } else {
        /* Intialize the interpreter. */
        let mut interpreter = BrainfuckInterpreter::default();

        /* Collect arguments and join them. */
        let brainfuck_program = arguments[1..].join("");

        /* Generate tokens from the arguments. */
        let tokens = BrainfuckToken::generate_tokens(&brainfuck_program);

        /* Execute the program. */
        interpreter.execute(&tokens);
    }
}
