use std::convert::TryFrom;
use std::marker::PhantomData;
use stack::Stack;
use num::Float;

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

pub trait Operate<T> {
    type Err;
    fn operate(stack: &mut Stack<T>) -> Result<(), Self::Err>;
}

// pub trait FromOperate<T> {
//     type Err;
//     fn operate(stack: &mut Stack<T>) -> Result<(), Self::Err>;
// }

#[derive(Debug)]
pub enum Arithm<T, O: Operate<T>> {
    Operand(T),
    Operator(O),
}

pub struct Expression<T, O: Operate<T>>(Vec<Arithm<T, O>>);

impl<'a, T, O: Operate<T>> TryFrom<&'a str> for Expression<T, O> {
    type Err = (); // TODO change this
    fn try_from(expr: &'a str) -> Result<Self, Self::Err> {
        // let mut arithm = Vec::new();

        for token in expr.split_whitespace() {
            // arithm.push(Arithm::Operand(0))
        }

        Ok(Expression(Vec::new())) // TODO change this
    }
}

// TODO move this elsewhere !!!
#[derive(Debug)]
pub enum FloatOperator<T: Float> {
    Add(PhantomData<T>),
    Sub(PhantomData<T>),
    Mul(PhantomData<T>),
    Div(PhantomData<T>),
}

impl<T: Float> Operate<T> for FloatOperator<T> {
    type Err = ();
    fn operate(stack: &mut Stack<T>) -> Result<(), Self::Err> {
        Ok(())
    }
}
