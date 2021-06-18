#[cfg(test)]
mod tests {
    use std::path::Path;
    use javid::{read_file, tokenizer, Token, Lexer};

    #[test]
    fn test_tokenization() {
        let current_path = file!();
        let parent_directory = Path::new(current_path).parent().unwrap().parent().unwrap();
        
        let path_to_truth = parent_directory.join("data").join("truth.txt");
        let expected = read_file(path_to_truth.as_path());
        let lines = expected.lines();
        let mut expected_array = vec![];

        for line in lines {
            let mut words = line.split(" ");
            let token = words.next().unwrap();
            let name = words.next().unwrap();
            
            let token = match token {
                "eof" => Token::Eof,
                "equal" => Token::Equal,
                "greater" => Token::Greater,
                "identifier" => Token::Identifier(String::from(&name[1..name.len() - 1])),
                "int" => Token::Int,
                "l_brace" => Token::LBrace,
                "l_paren" => Token::LParen,
                "minusminus" => Token::MinusMinus,
                "numeric_constant" => Token::NumericConstant(name[1..name.len() - 1].parse::<usize>().unwrap()),
                "plusequal" => Token::PlusEqual,
                "r_brace" => Token::RBrace,
                "return_key" => Token::Return,
                "r_paren" => Token::RParen,
                "semi" => Token::Semi,
                "while_key" => Token::While,
                _ => panic!("Unknown token"),
            };
            expected_array.push(token);
        }
        
        let path_to_input = parent_directory.join("data").join("example.c");
        let found: Lexer = tokenizer(path_to_input.as_path());
        let token_vec: Vec<Token> = tokenizer(path_to_input.as_path()).collect();

        assert_eq!(token_vec.len(), expected_array.len());
        let zippedIter = expected_array.into_iter().zip(found.into_iter());
        for (expected,actual) in zippedIter {
            assert_eq!(expected,actual);
        }
    }
}