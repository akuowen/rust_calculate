use crate::calc::ast::Node;
use crate::calc::error::{CalcError, CalcResult};
use crate::calc::token::{OperatorPrecedence, Token};
use crate::calc::tokenizer::Tokenizer;

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    current_token: Token,
}


impl<'a> Parser<'a> {
    pub fn new(expression: &'a str) -> CalcResult<Self> {
        let mut tokenizer = Tokenizer::new(expression);
        let current_token = tokenizer
            .next()
            .ok_or_else(|| CalcError::UnexpectedChar(tokenizer.get_unexpected_char().unwrap()))?;
        Ok(Parser {
            tokenizer,
            current_token,
        })
    }

    pub fn parse(&self) -> CalcResult<Node> {
        todo!()
    }
}


impl<'a> Parser<'a> {

    
    ///
    /// [
    //     {
    //         "Number": "1"
    //     },
    //     "Add",
    //     {
    //         "Number": "2"
    //     },
    //     "Mul",
    //     {
    //         "Function": {
    //             "function_prefix": "nvl",
    //             "args": [
    //                 [
    //                     {
    //                         "Function": {
    //                             "function_prefix": "abs",
    //                             "args": [
    //                                 [
    //                                     {
    //                                         "Number": "1"
    //                                     },
    //                                     "Add",
    //                                     {
    //                                         "Number": "2"
    //                                     },
    //                                     "Mul",
    //                                     {
    //                                         "Number": "3"
    //                                     },
    //                                     "Add",
    //                                     "LeftMidParen",
    //                                     "LeftSmallParen",
    //                                     {
    //                                         "Number": "1"
    //                                     },
    //                                     "Add",
    //                                     {
    //                                         "Number": "3"
    //                                     },
    //                                     "RightSmallParen",
    //                                     "Div",
    //                                     {
    //                                         "Number": "2"
    //                                     },
    //                                     "RightMidParen"
    //                                 ],
    //                                 [
    //                                     {
    //                                         "Number": "0"
    //                                     }
    //                                 ]
    //                             ]
    //                         }
    //                     }
    //                 ],
    //                 [
    //                     {
    //                         "Number": "0"
    //                     }
    //                 ]
    //             ]
    //         }
    //     },
    //     "EOF"
    // ]
    /// 
    fn parse_expression(&self,operation_precedence:OperatorPrecedence)->CalcResult<Node>{
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::calc::parser::Parser;

    #[test]
    fn test_new_parser() {
        let result = Parser::new("1 + 2 * nvl < abs < 1 + 2 * 3 + [ ( 1+ 3 ) / 2 ] ) , 0 > , 0 >");
        let _ = result.is_err_and(|_| panic!("test_new_parser error"));
    }
}
