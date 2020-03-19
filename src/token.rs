#[derive(PartialEq, Debug)]
pub enum BrainfuckToken {
    IncrementPointer = 1,
    DecrementPointer = 2,
    IncrementValue = 3,
    DecrementValue = 4,
    PrintPointer = 5,
    GetCharacterInput = 6,
    StartLoop = 7,
    EndLoop = 8,
    Comment = 9,
}
impl From<&u8> for BrainfuckToken {
    fn from(token: &u8) -> Self {
        return match *token {
            b'[' => BrainfuckToken::StartLoop,
            b']' => BrainfuckToken::EndLoop,
            b'>' => BrainfuckToken::IncrementPointer,
            b'<' => BrainfuckToken::DecrementPointer,
            b'+' => BrainfuckToken::IncrementValue,
            b'-' => BrainfuckToken::DecrementValue,
            b'.' => BrainfuckToken::PrintPointer,
            b',' => BrainfuckToken::GetCharacterInput,
            /* Any character that isn't a BrainfuckToken by the spec is a comment. */
            _ => BrainfuckToken::Comment,
        };
    }
}

impl BrainfuckToken {
    pub fn generate_tokens(to_tokenize: &str) -> Vec<BrainfuckToken> {
        to_tokenize
            .as_bytes()
            .iter()
            .map(BrainfuckToken::from)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_token_matcher() {
        for i in 1..=255 as u8 {
            match i {
                b'[' => assert_eq!(BrainfuckToken::StartLoop, BrainfuckToken::from(&b'[')),
                b']' => assert_eq!(BrainfuckToken::EndLoop, BrainfuckToken::from(&b']')),
                b'>' => assert_eq!(
                    BrainfuckToken::IncrementPointer,
                    BrainfuckToken::from(&b'>')
                ),
                b'<' => assert_eq!(
                    BrainfuckToken::DecrementPointer,
                    BrainfuckToken::from(&b'<')
                ),
                b'+' => assert_eq!(BrainfuckToken::IncrementValue, BrainfuckToken::from(&b'+')),
                b'-' => assert_eq!(BrainfuckToken::DecrementValue, BrainfuckToken::from(&b'-')),
                b'.' => assert_eq!(BrainfuckToken::PrintPointer, BrainfuckToken::from(&b'.')),
                b',' => assert_eq!(
                    BrainfuckToken::GetCharacterInput,
                    BrainfuckToken::from(&b',')
                ),
                comments => assert_eq!(
                    BrainfuckToken::Comment,
                    BrainfuckToken::from(&comments),
                    "Testing the conversion from byte to token."
                ),
            }
        }
    }

    #[test]
    fn test_token_generation() {
        const TEST_STRING: &str = "[]><+-.,\n\0";
        let expected = vec![
            BrainfuckToken::StartLoop,
            BrainfuckToken::EndLoop,
            BrainfuckToken::IncrementPointer,
            BrainfuckToken::DecrementPointer,
            BrainfuckToken::IncrementValue,
            BrainfuckToken::DecrementValue,
            BrainfuckToken::PrintPointer,
            BrainfuckToken::GetCharacterInput,
            BrainfuckToken::Comment,
            BrainfuckToken::Comment,
        ];

        assert_eq!(
            BrainfuckToken::generate_tokens(TEST_STRING),
            expected,
            "Testing the ability to turn a string into a vector of tokens."
        );
    }
}
