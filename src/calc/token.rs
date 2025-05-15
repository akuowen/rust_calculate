use rust_decimal::Decimal;
use std::fmt::{Display, Formatter};
use serde::Serialize;

/// Represents a token in the calculator's syntax.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum Token {
    // 数字
    Number(Decimal),
    // 变量
    Variable(String),
    // 嵌套函数结构
    Function {
        function_prefix: String,
        args: Vec<Vec<Token>>,
    },
    // +
    Add,
    // -
    Sub,
    // x
    Mul,
    // /
    Div,
    // ^
    Caret,
    // (
    LeftSmallParen,
    // )
    RightSmallParen,
    // [
    LeftMidParen,
    // ]
    RightMidParen,
    // {
    LeftBigParen,
    // }
    RightBigParen,
    // <
    LeftFuncParen,
    // >
    RightFuncParen,
    // ,
    Comma,
    EOF,
}

#[allow(unused)]
impl Token {
    /// Returns the operator precedence of the token.
    ///
    /// The precedence is used to determine the order of operations when
    /// evaluating an expression. The higher the precedence, the more
    /// tightly an operator binds to its operands. For example, the
    /// multiplication operator (Mul) has a higher precedence than the
    /// addition operator (Add) because it binds more tightly to its
    /// operands.
    ///
    /// The precedence levels are as follows:
    ///
    /// - `Default`: Any token that is not an operator. This includes
    ///   numbers, variables, and parentheses.
    /// - `AddOrSubtract`: The addition and subtraction operators.
    /// - `MultiplyOrDivide`: The multiplication, division, and modulus
    ///   operators.
    /// - `Power`: The power operator.
    /// - `Function`: Function calls.
    /// - `Negative`: The negative operator.
    ///
    /// # Examples
    ///
    pub fn get_precedence(&self) -> OperatorPrecedence {
        match self {
            Self::Add | Self::Sub => OperatorPrecedence::AddOrSubtract,
            Self::Mul | Self::Div => OperatorPrecedence::MultiplyOrDivide,
            Self::Caret => OperatorPrecedence::Power,
            Self::Function { .. } => OperatorPrecedence::Function,
            _ => OperatorPrecedence::Default,
        }
    }
}

impl Display for Token {
    /// Formats a `Token` into a string for debugging purposes.
    ///
    /// The output is a string that represents the token in a way that is
    /// easy to read and understand. For example, the token `Token::Number(1.0)`
    /// is formatted as the string `"1.0"`.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::calc::token::Token;
    /// use rust_decimal::Decimal;
    ///
    /// let token = Token::Number(Decimal::from(42));
    /// assert_eq!(format!("{}", token), "42");
    ///
    /// let token = Token::Add;
    /// assert_eq!(format!("{}", token), "+");
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            // Format numeric values directly
            Self::Number(n) => write!(f, "{}", n),
            
            // Format variable names as-is
            Self::Variable(var) => f.write_str(var),
            
            // Format basic operators with their symbols
            Self::Add => f.write_str("+"),
            Self::Sub => f.write_str("-"),
            Self::Mul => f.write_str("×"),  // Using multiplication symbol instead of asterisk
            Self::Div => f.write_str("÷"),  // Using division symbol instead of slash
            
            // Format function calls as: function_name<(arg1), (arg2), ...>
            Self::Function { function_prefix, args } => {
                // Write function name and opening bracket
                write!(f, "{}<", function_prefix)?;
                
                // Format each argument list
                for (i, arg) in args.iter().enumerate() {
                    // Add comma separator between arguments
                    if i > 0 {
                        f.write_str(",")?;
                    }
                    
                    // Wrap each argument list in parentheses
                    // f.write_str("(")?;
                    
                    // Format tokens within each argument
                    for (j, t) in arg.iter().enumerate() {
                        // Add space between tokens in the same argument
                        if j > 0 {
                            f.write_str(" ")?;
                        }
                        write!(f, "{}", t)?;
                    }
                    
                    // Close the argument parenthesis
                    // f.write_str(")")?;
                }
                
                // Close the function bracket
                f.write_str(">")
            },
            
          
            
            // Format other operators and symbols
            Self::Caret => f.write_str("^"),
            
            // Format different types of parentheses
            Self::LeftSmallParen => f.write_str("("),
            Self::RightSmallParen => f.write_str(")"),
            Self::LeftMidParen => f.write_str("["),
            Self::RightMidParen => f.write_str("]"),
            Self::LeftBigParen => f.write_str("{"),
            Self::RightBigParen => f.write_str("}"),
            
            // Format other syntax elements
            Self::Comma => f.write_str(","),
            Self::LeftFuncParen => f.write_str("<"),
            Self::RightFuncParen => f.write_str(">"),
            
            // Format end-of-file token
            Self::EOF => f.write_str("EOF"),
        }
    }
}

#[allow(unused)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum OperatorPrecedence {
    Default,
    AddOrSubtract,
    MultiplyOrDivide,
    Power,
    Negative,
    Function,
}
