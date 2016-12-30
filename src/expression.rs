use std::convert::{From, TryFrom, TryInto};
use std::ops::{Deref, DerefMut};
use std::str::FromStr;
use std::fmt;
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
    InvalidToken(A, B),
}

impl<'a, T, O> Expression<T, O>
    where T: FromStr,
          O: Operate<T> + TryFrom<&'a str>
{
    fn check_validity(expr: &Vec<Arithm<T, O>>)
        -> Result<(), ExprResult<<T as FromStr>::Err, <O as TryFrom<&'a str>>::Err>> {
        use self::ExprResult::*;
        let mut num_operands: usize = 0;
        for arithm in expr {
            match *arithm {
                Arithm::Operand(ref operand) => num_operands += 1,
                Arithm::Operator(ref operator) => {
                    let needed = operator.operands_needed();
                    num_operands = num_operands.checked_sub(needed).ok_or(NotEnoughOperand)?;
                    num_operands += operator.operands_generated();
                },
            }
        }
        match num_operands {
            0 => Err(NotEnoughOperand),
            1 => Ok(()),
            _ => Err(TooManyOperands),
        }
    }
}

impl<'a, T, O> TryFrom<&'a str> for Expression<T, O>
    where T: FromStr,
          O: Operate<T> + TryFrom<&'a str>
{
    type Err = ExprResult<<T as FromStr>::Err, <O as TryFrom<&'a str>>::Err>;
    fn try_from(expr: &'a str) -> Result<Self, Self::Err> {
        use self::ExprResult::*;
        let mut final_expr = Vec::new();
        let mut num_operands: usize = 0;
        for token in expr.split_whitespace() {
            let arithm = match token.parse() {
                Ok(operand) => Arithm::Operand(operand),
                Err(operand_err) => {
                    match TryInto::<O>::try_into(token) {
                        Ok(operator) => Arithm::Operator(operator),
                        Err(operator_err) => return Err(InvalidToken(operand_err, operator_err)),
                    }
                }
            };
            final_expr.push(arithm);
        }
        Expression::check_validity(&final_expr).map(|_| Expression(final_expr))
    }
}

impl<T, O: Operate<T>> From<Vec<Arithm<T, O>>> for Expression<T, O> {
    fn from(expr: Vec<Arithm<T, O>>) -> Self {
        Expression(expr)
    }
}

impl<T, O: Operate<T>> Into<Vec<Arithm<T, O>>> for Expression<T, O> {
    fn into(self) -> Vec<Arithm<T, O>> {
        self.0
    }
}

impl<T, O: Operate<T>> Deref for Expression<T, O> {
    type Target = Vec<Arithm<T, O>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, O: Operate<T>> DerefMut for Expression<T, O> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T, O> fmt::Display for Expression<T, O>
    where T: fmt::Display,
          O: Operate<T> + fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let len = self.0.len();
        for (i, arithm) in self.0.iter().enumerate() {
            match *arithm {
                Arithm::Operand(ref operand) => operand.fmt(f)?,
                Arithm::Operator(ref operator) => operator.fmt(f)?,
            }
            if i != len - 1 {
                f.write_str(" ")?
            }
        }
        Ok(())
    }
}
