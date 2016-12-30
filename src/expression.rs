use std::convert::{TryFrom, TryInto};
use std::str::FromStr;
use stack::Stack;
use operate::Operate;

#[derive(Debug, Copy, Clone)]
pub enum Arithm<T, O: Operate<T>> {
    Operand(T),
    Operator(O),
}

#[derive(Debug)]
pub struct Expression<T, O: Operate<T>>(Vec<Arithm<T, O>>);

impl<T: Copy, O: Operate<T> + Copy> Expression<T, O> {
    pub fn operate(&self) -> Result<T, O::Err> {
        let mut stack = Stack::new();
        for arithm in &self.0 {
            match *arithm {
                Arithm::Operand(operand) => stack.push(operand),
                Arithm::Operator(operator) => operator.operate(&mut stack)?,
            }
        }
        Ok(stack.pop().unwrap())
    }
}

#[derive(Debug, PartialEq)]
pub enum ExprResult<A, B> {
    TooManyOperands,
    NotEnoughOperand,
    InvalidToken(A, B)
}

use self::ExprResult::*;

impl<'a, T, O> TryFrom<&'a str> for Expression<T, O>
               where T: FromStr,
                     O: Operate<T> + TryFrom<&'a str> {
    type Err = ExprResult<<T as FromStr>::Err, <O as TryFrom<&'a str>>::Err>;
    fn try_from(expr: &'a str) -> Result<Self, Self::Err> {
        let mut expression = Vec::new();
        let mut operands: usize = 0;
        for token in expr.split_whitespace() {
            let arithm = match token.parse() {
                Ok(operand) => {
                    operands += 1;
                    Arithm::Operand(operand)
                },
                Err(operand_err) => match TryInto::<O>::try_into(token) {
                    Ok(operator) => {
                        let needed = operator.operands_needed();
                        operands = operands.checked_sub(needed)
                                           .ok_or(NotEnoughOperand)?;
                        operands += operator.operands_generated();
                        Arithm::Operator(operator)
                    },
                    Err(operator_err) => {
                        return Err(InvalidToken(operand_err, operator_err))
                    }
                },
            };
            expression.push(arithm);
        }
        match operands {
            0 => Err(NotEnoughOperand),
            1 => Ok(Expression(expression)),
            _ => Err(TooManyOperands)
        }
    }
}
