use std::convert::{TryFrom, TryInto};
use std::str::FromStr;
use std::marker::PhantomData;
use stack::Stack;
use num::Float;
use ::pop_two_operands;

pub trait Operate<T> {
    type Err;
    fn operate(self, stack: &mut Stack<T>) -> Result<(), Self::Err>;
}

#[derive(Debug, Copy, Clone)]
pub enum Arithm<T, O: Operate<T>> {
    Operand(T),
    Operator(O),
}

#[derive(Debug)]
pub struct Expression<T, O: Operate<T>>(Vec<Arithm<T, O>>);

impl<T: Copy, O: Operate<T> + Copy> Expression<T, O> {
    pub fn operate(&self) -> Result<T, ()> { // TODO change this
        let mut stack = Stack::new();
        for arithm in &self.0 {
            match *arithm {
                Arithm::Operand(operand) => stack.push(operand),
                Arithm::Operator(operator) => {
                    if let Err(_) = operator.operate(&mut stack) { // TODO change this
                        return Err(())
                    }
                },
            }
        }
        stack.pop().ok_or(())
    }
}

impl<'a, T, O> TryFrom<&'a str> for Expression<T, O>
               where T: FromStr,
                     O: Operate<T> + TryFrom<&'a str> {
    type Err = (); // TODO change this
    fn try_from(expr: &'a str) -> Result<Self, Self::Err> {
        let mut expression = Vec::new();
        for token in expr.split_whitespace() {
            // TODO make this more rusty !
            let arithm: Arithm<T, O> = match token.parse() {
                Ok(operand) => Arithm::Operand(operand),
                Err(operand_err) => match token.try_into() {
                    Ok(operator) => Arithm::Operator(operator),
                    Err(operator_err) => return Err(()), // TODO change this
                },
            };
            expression.push(arithm);
        }
        Ok(Expression(expression)) // TODO change this
    }
}

// Add  (+)     pop 2, push 1
// Sub  (-)     pop 2, push 1
// Mul  (*)     pop 2, push 1
// Div  (/)     pop 2, push 1
// Neg  (neg)   pop 1, push 1
// Sqrt (sqrt)  pop 1, push 1
// Pow  (pow)   pop 1, push 1
// Log2 (log2)  pop 1, push 1
// Exp  (exp)   pop 1, push 1
// Swap (swap)  pop 2, push 2
// Zero (zero)  pop 0, push 1
// One  (zero)  pop 0, push 1

// TODO move this elsewhere !!!
#[derive(Debug, Copy, Clone)]
pub enum FloatOperator<T: Float> {
    Add(PhantomData<T>),
    Sub(PhantomData<T>),
    Mul(PhantomData<T>),
    Div(PhantomData<T>),
}

impl<T: Float> Operate<T> for FloatOperator<T> {
    type Err = ();
    fn operate(self, stack: &mut Stack<T>) -> Result<(), Self::Err> {
        match self {
            FloatOperator::Add(_) => {
                let (a, b) = pop_two_operands(stack).ok_or(())?;
                Ok(stack.push(a + b))
            },
            FloatOperator::Sub(_) => {
                let (a, b) = pop_two_operands(stack).ok_or(())?;
                Ok(stack.push(b - a))
            },
            FloatOperator::Mul(_) => {
                let (a, b) = pop_two_operands(stack).ok_or(())?;
                Ok(stack.push(a * b))
            },
            FloatOperator::Div(_) => {
                let (a, b) = pop_two_operands(stack).ok_or(())?;
                Ok(stack.push(b / a))
            },
        }
    }
}

impl<'a, T: Float> TryFrom<&'a str> for FloatOperator<T> {
    type Err = (); // TODO change this
    fn try_from(expr: &'a str) -> Result<Self, Self::Err> {
        match expr {
            "+" => Ok(FloatOperator::Add(PhantomData::default())),
            "-" => Ok(FloatOperator::Sub(PhantomData::default())),
            "*" => Ok(FloatOperator::Mul(PhantomData::default())),
            "/" => Ok(FloatOperator::Div(PhantomData::default())),
            _ => Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;
    use expression::{Expression, FloatOperator};

    #[test]
    fn simple_addition() {
        let expr: Expression<f32, FloatOperator<_>> = "3 4 +".try_into().unwrap();
        assert_eq!(expr.operate(), Ok(7.0));
    }

    #[test]
    fn simple_substraction() {
        let expr: Expression<f32, FloatOperator<_>> = "4 3 -".try_into().unwrap();
        assert_eq!(expr.operate(), Ok(1.0));
    }

    #[test]
    fn simple_multiplication() {
        let expr: Expression<f32, FloatOperator<_>> = "3 4 *".try_into().unwrap();
        assert_eq!(expr.operate(), Ok(12.0));
    }

    #[test]
    #[should_panic]
    fn simple_wrong_expression() {
        let expr: Expression<f32, FloatOperator<_>> = "3 4 + &".try_into().unwrap();
    }
}
