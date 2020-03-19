use brainfuck::token::BrainfuckToken;
use brainfuck::interpreter::BrainfuckInterpreter;

#[test]
fn test_simple_program() {
    /* Simple test program to test functionality:
    1. `++++` will set memory[0] to 4
    2. `[>+<-]`  will do the following:
        a. It will increment the memory index: memory[1]
        b. Increment memory[1] by `1`
        c. Decrement the memory index: memory[0]
        d. Decrement memory[0] by `1`
    In total: The program will make the memory layout:

    1. memory { 0 => 4, 1 => 0 }
    2. memory { 0 => 0, 1 => 4} */   

    const TEST_PROGRAM: &str = "++++[>+<-]";
    let tokens = BrainfuckToken::generate_tokens(TEST_PROGRAM);
    let mut interpreter = BrainfuckInterpreter::default();

    interpreter.execute(&tokens);

    assert_eq!(interpreter.memory_index, 0);
    assert_eq!(interpreter.memory[0], 0);
    assert_eq!(interpreter.memory[1], 4);
    assert_eq!(interpreter.token_index, 10);
}

#[test]
fn test_hello_world() {
    /* Testing hello world, from the wikipedia page:
    * https://en.wikipedia.org/wiki/Brainfuck#Hello_World! */
    let hello_world_program = include_str!("hello_world.bf");

    let tokens = BrainfuckToken::generate_tokens(hello_world_program);
    let mut interpreter = BrainfuckInterpreter::default();

    interpreter.execute(&tokens);

    assert_eq!(true, true);
}