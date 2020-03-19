use super::token::BrainfuckToken;

pub struct BrainfuckInterpreter {
    pub memory: Vec<i32>,
    pub memory_index: usize,
    pub token_index: usize,
}
impl Default for BrainfuckInterpreter {
    fn default() -> BrainfuckInterpreter {
        BrainfuckInterpreter {
            memory: Vec::with_capacity(1000),
            memory_index: 0,
            token_index: 0,
        }
    }
}
impl BrainfuckInterpreter {
    fn print_memory_at_index(&self) {
        /* Get u32 representation of the memory at the current index. */
        let character_code = *self.memory.get(self.memory_index).unwrap_or(&0) as u32;

        /* Print the value. If it fails to convert use a null-byte. */
        print!("{}", std::char::from_u32(character_code).unwrap_or('\0'))
    }

    fn increase_memory_as_needed(&mut self) {
        /* If the current size of memory is less then the pointer expects to be available */
        if self.memory.len() <= self.memory_index {
            /* Resize the vector to meet the expectation. */
            self.memory.resize(self.memory_index + 1, 0);
        }
    }

    fn read_character_into_memory(&mut self) {
        self.increase_memory_as_needed();

        /* Set the memory at the current index */
        let mut input_text = String::new();
        std::io::stdin()
            .read_line(&mut input_text)
            .expect("failed to read from stdin");

        let trimmed = input_text.trim();
        match trimmed.parse::<i32>() {
            Ok(input) => self.memory[self.memory_index] = input,
            Err(..) => self.memory[self.memory_index] = 0,
        };
    }

    fn increment_memory_at_index(&mut self) {
        /* If the current size of memory is less then the pointer expects to be available */
        self.increase_memory_as_needed();
        /* Increment the value of the memory at the current index */
        *self.memory.get_mut(self.memory_index).unwrap() += 1;
    }

    fn decrement_memory_at_index(&mut self) {
        /* If the current size of memory is less then the pointer expects to be available */
        self.increase_memory_as_needed();
        /* Increment the value of the memory at the current index */
        *self.memory.get_mut(self.memory_index).unwrap() -= 1;
    }

    fn break_loop(&mut self, message: &[BrainfuckToken]) {
        /* Our current token is '['. If we assume the the following:
        '[' add +1 to the loop counter
        ']' add -1 to the loop counter
        Then when the the counter hits 0 we have found
        the closing tag to match our opening tag. */
        let mut loop_counter = 0;
        for (modifier, token) in message[self.token_index..].iter().enumerate() {
            match token {
                BrainfuckToken::StartLoop => loop_counter += 1,
                BrainfuckToken::EndLoop => loop_counter -= 1,
                _ => (),
            }
            if loop_counter == 0 {
                self.token_index += modifier;
                return;
            }
        }
    }
    fn rewind_loop(&mut self, message: &[BrainfuckToken]) {
        /* Assuming the currrent indice is 1, our current
        token is ']'. If we assume the the following:
        ']' add +1 to the loop counter
        '[' add -1 to the loop counter
        Then when the the counter hits 0 we have found
        the open tag to match our closing tag. */
        let mut loop_counter = 1;
        for (modifier, token) in message[..self.token_index].iter().rev().enumerate() {
            match token {
                BrainfuckToken::EndLoop => loop_counter += 1,
                BrainfuckToken::StartLoop => loop_counter -= 1,
                _ => (),
            }
            if loop_counter == 0 {
                /* Modifier is 0-indexed, so we need to change this otherwise
                when 'self.execute' finishes it will increment past the
                first instruction. */
                self.token_index -= modifier + 1;
                return;
            }
        }
    }

    fn begin_while(&mut self, message: &[BrainfuckToken]) {
        /* Brainfuck treats the loop as a `while (current_memory)`.
        We check for the value of memory at the current index.
        If the value is unset, we substitute the value `0` in. */
        if self.memory.get(self.memory_index).unwrap_or(&0) <= &0 {
            /* If the value is LESS than or equal to 0 it evaluates
            to false, and thus the while condition because
            `while(false)` which will never execute. We call
            `self.break_loop` seek the matching ']' tag. */
            self.break_loop(&message);
        }
    }

    fn end_while(&mut self, message: &[BrainfuckToken]) {
        /* The end of the `while (current_memory)` statement.
        We check for the value of memory at the current index.
        If the value is unset, we substitute the value `0` in. */
        if self.memory.get(self.memory_index).unwrap_or(&0) > &0 {
            /* If the value is GREATER than 0 it evaluates to true, and
            thus the while condition because `while(true)`. We call `
            self.rewind_loop` seek the matching '[' tag to find the start
            of this loop. */
            self.rewind_loop(&message);
        }
    }

    fn parse(&mut self, message: &[BrainfuckToken]) {
        match message[self.token_index] {
            BrainfuckToken::IncrementPointer => self.memory_index += 1,
            BrainfuckToken::DecrementPointer => self.memory_index -= 1,
            BrainfuckToken::IncrementValue => self.increment_memory_at_index(),
            BrainfuckToken::DecrementValue => self.decrement_memory_at_index(),
            BrainfuckToken::PrintPointer => self.print_memory_at_index(),
            BrainfuckToken::GetCharacterInput => self.read_character_into_memory(),
            BrainfuckToken::StartLoop => self.begin_while(&message),
            BrainfuckToken::EndLoop => self.end_while(&message),
            /* Anything that isn't a brainfuck token by default gets interpreted as being a comment. */
            BrainfuckToken::Comment => (),
        }
    }

    pub fn execute(&mut self, message: &[BrainfuckToken]) {
        while self.token_index < message.len() {
            self.parse(message);
            self.token_index += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn test_vector() -> Vec<BrainfuckToken> {
        vec![
            BrainfuckToken::IncrementPointer,
            BrainfuckToken::Comment,
            BrainfuckToken::StartLoop,
            BrainfuckToken::IncrementPointer,
            BrainfuckToken::StartLoop,
            BrainfuckToken::EndLoop,
            BrainfuckToken::IncrementPointer,
            BrainfuckToken::EndLoop,
            BrainfuckToken::DecrementValue,
        ]
    }
    #[test]
    fn interpreter_defaults_are_expected() {
        let interpreter = BrainfuckInterpreter::default();
        assert_eq!(
            interpreter.token_index, 0,
            "Testing token index default value"
        );
        assert_eq!(
            interpreter.memory_index, 0,
            "Testing memoriy index default value"
        );
        assert_eq!(
            interpreter.memory.capacity(),
            1000,
            "Testing interpreter default memory size"
        );
    }

    #[test]
    fn interpreter_increment_value_overflowing_memory_bounds() {
        let mut interpreter = BrainfuckInterpreter::default();
        interpreter.memory_index = interpreter.memory.capacity() + 1;
        interpreter.increment_memory_at_index();
        match interpreter.memory.get(interpreter.memory_index) {
            Some(value) => assert_eq!(*value, 1, "Increment value outside memory boundary"),
            None => panic!("Failed to increment memory past the boundary of the vector."),
        }
    }

    #[test]
    fn interpreter_decrement_value_overflowing_memory_bounds() {
        let mut interpreter = BrainfuckInterpreter::default();
        interpreter.memory_index = interpreter.memory.capacity() + 1;
        interpreter.decrement_memory_at_index();
        match interpreter.memory.get(interpreter.memory_index) {
            Some(value) => assert_eq!(*value, -1, "Decrement value outside memory boundary"),
            None => panic!("Failed to decrement memory past the boundary of the vector."),
        }
    }

    #[test]
    fn interpreter_break_loop_index_check() {
        let mut interpreter = BrainfuckInterpreter::default();
        let tokens = test_vector();
        /* First start loop */
        interpreter.token_index = 2;
        interpreter.break_loop(&tokens);
        assert_eq!(interpreter.token_index, 7);
    }

    #[test]
    fn interpreter_rewind_loop_index_check() {
        let mut interpreter = BrainfuckInterpreter::default();
        let tokens = test_vector();
        /* First start loop */
        interpreter.token_index = 7;
        interpreter.rewind_loop(&tokens);
        assert_eq!(interpreter.token_index, 2);
    }

    #[test]
    fn interpreter_parsing_comment_does_nothing() {
        const EXPECTED_TOKEN_INDEX: usize = 1;
        const EXPECTED_MEMORY_INDEX: usize = 1;
        const EXPECTED_MEMORY_VALUE: i32 = 1;
        let mut interpreter = BrainfuckInterpreter::default();
        interpreter.token_index = EXPECTED_TOKEN_INDEX;

        let tokens = test_vector();
        interpreter.memory_index += 1;
        interpreter.increment_memory_at_index();
        interpreter.parse(&tokens);

        assert_eq!(
            interpreter.token_index, EXPECTED_TOKEN_INDEX,
            "Parse comment -- token index check"
        );
        assert_eq!(
            interpreter.memory_index, EXPECTED_MEMORY_INDEX,
            "Parse comment -- memory index check"
        );
        assert_eq!(
            interpreter.memory[EXPECTED_MEMORY_INDEX], EXPECTED_MEMORY_VALUE,
            "Parse comment -- value at memory index check"
        );
    }
}
