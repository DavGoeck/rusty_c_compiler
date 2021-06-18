#[cfg(test)]
mod tests {
    use std::path::Path;
    use javid::{read_file, tokenize, Token};

    #[test]
    fn test_tokenization() {
        let current_path = file!();
        let parent_directory = match Path::new(current_path).parent()
            {
                None => panic!("No parent found!"),
                Some(parent_path) => parent_path,
            };
        
        let path_to_truth = parent_directory.join("data").join("truth.txt");
        let expected = read_file(path_to_truth.as_path());
        let lines = expected.lines();
        let mut expected_array = vec![];

        for line in lines {
            
            let token = match line.split(" ").nth(0).unwrap() {
                "eof" => Token::eof,
                "equal" => Token::equal,
                "greater" => Token::greater,
                "identifier" => Token::identifier,
                "int" => Token::int,
                "l_brace" => Token::l_brace,
                "l_paren" => Token::l_paren,
                "minusminus" => Token::minusminus,
                "numeric_constant" => Token::numeric_constant,
                "plusequal" => Token::plusequal,
                "r_brace" => Token::r_brace,
                "return_key" => Token::return_key,
                "r_paren" => Token::r_paren,
                "semi" => Token::semi,
                "while_key" => Token::while_key,
                _ => panic!("Unknown token"),
            };
            expected_array.push(token);
        }
        
        let path_to_input = parent_directory.join("data").join("example.c");
        let found = tokenize(path_to_input.as_path());

        assert_eq!(expected_array.len(), found.len());
        let zippedIter = expected_array.into_iter().zip(found.into_iter());
        for (a,b) in zippedIter {
            assert_eq!(a,b);
        }
    }
}