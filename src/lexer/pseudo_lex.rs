macro_rules! pseudo_lex {
    ($t:ty) => {
        |input: &'a str| {
            let (i, token) = $crate::lexer::token(input)?;
            let array = [token];
            let tokens = $crate::lexer::Tokens(array);
            match eat!($t)(tokens) {
                Ok((_, token)) => Ok(i, token),
                Err(_) => Err(nom::Err::Error(nom::error::Error {
                    i,
                    code: nom::error::ErrorKind::Char,
                }))
            }
        }
    }
}
