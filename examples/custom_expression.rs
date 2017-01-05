extern crate ripin;

use std::env;
use std::marker::PhantomData;
use ripin::{Stack, pop_two_operands, TryFromRef};
use ripin::expression::Expression;
use ripin::evaluate::Evaluate;

// Implementing Expression for a new specific type
// is not a difficult thing to do:

// First you will need something that
// can be understand as an Operand type, like float or integer.
// The type you choose need to implement TryFromRef.
#[derive(Debug, Copy, Clone)]
enum MyOperand {
    Number1,
    Number2,
}

impl<'a> TryFromRef<&'a str> for MyOperand {
    type Err = MyOperandErr<&'a str>;
    fn try_from_ref(s: &&'a str) -> Result<Self, Self::Err> {
        match *s {
            "1" => Ok(MyOperand::Number1),
            "2" => Ok(MyOperand::Number2),
            _ => Err(MyOperandErr::InvalidToken(s))
        }
    }
}

// Secondly an Evaluator type to make evaluation
// of Operands and generate other ones on the stack.
// It needs to implement TryFromRef too.
#[derive(Debug, Copy, Clone)]
enum MyEvaluator<T> {
    Add,
    Sub,
    _Phantom(PhantomData<T>) // make it generic but
                             // don't own the generic T type
}

impl<'a, T> TryFromRef<&'a str> for MyEvaluator<T> {
    type Err = MyOperandErr<&'a str>;
    fn try_from_ref(s: &&'a str) -> Result<Self, Self::Err> {
        match *s {
            "+" => Ok(MyEvaluator::Add),
            "-" => Ok(MyEvaluator::Sub),
            _ => Err(MyOperandErr::InvalidToken(s))
        }
    }
}

// A clear error struct/enum is really important for the parsing part
#[derive(Debug)]
enum MyOperandErr<T> {
    InvalidToken(T),
}

// Be careful both needs to implement the same TryFromRef Trait signature !
// If the Operand type works with TryFromRef<&str>
// then the Evaluator needs the same TryFromRef signature.

// A clear error struct/enum is really important for the evaluation part
#[derive(Debug)]
enum MyEvalErr<T> {
    CannotAddOperands(T, T),
    CannotSubOperands(T, T),
    NotEnoughOperands
}

// The last step is to implement the Evaluate trait on your custom Evaluator.
// Evaluations are done with this trait.
impl Evaluate<MyOperand> for MyEvaluator<MyOperand> {
    type Err = MyEvalErr<MyOperand>;

    fn operands_needed(&self) -> usize {
        match *self {
            MyEvaluator::Add | MyEvaluator::Sub => 2,
            _ => unreachable!(), // _Phantom
        }
    }
    fn operands_generated(&self) -> usize {
        match *self {
            MyEvaluator::Add | MyEvaluator::Sub => 1,
            _ => unreachable!(), // _Phantom
        }
    }

    fn evaluate(self, stack: &mut Stack<MyOperand>) -> Result<(), Self::Err> {
        let (a, b) = pop_two_operands(stack).ok_or(MyEvalErr::NotEnoughOperands)?;
        match self {
            MyEvaluator::Add => {
                match (a, b) {
                    (MyOperand::Number1, MyOperand::Number1) => {
                        Ok(stack.push(MyOperand::Number2))
                    },
                    _ => Err(MyEvalErr::CannotAddOperands(a, b)),
                }
            },
            MyEvaluator::Sub => {
                match (a, b) {
                    (MyOperand::Number2, MyOperand::Number1) => {
                        Ok(stack.push(MyOperand::Number1))
                    },
                    _ => Err(MyEvalErr::CannotSubOperands(a, b)),
                }
            }
            _ => unreachable!() // _Phantom
        }
    }
}

type MyExpression = Expression<MyOperand, MyEvaluator<MyOperand>>;

// Once you implement the TryFromRef trait on your “custom” types,
// make an iterator of it and give it to the Expression struct.
fn main() {
    let expr_str = env::args().nth(1).unwrap_or_else(|| {
        println!("Give me an expression as first argument!");
        "1 1 +".into()
    });

    let tokens = expr_str.split_whitespace();
    match MyExpression::from_iter(tokens) {
        Ok(expr) => println!("Evaluation of {:?} gives {:?}", expr_str, expr.evaluate()),
        Err(err) => println!("Parsing results in {:?}", err),
    }
}
