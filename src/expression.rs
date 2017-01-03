use std::fmt;
use stack::Stack;
use evaluate::Evaluate;
use try_from_ref::{TryFromRef, TryIntoRef};

/// Used to specify an `Operand` or an `Evaluator`.
#[derive(Debug, Copy, Clone)]
pub enum Arithm<T, O: Evaluate<T>> {
    Operand(T),
    Evaluator(O),
}

/// Interpret a [`Reverse Polish notated`] expression (cf. `3 4 +`).
///
/// `Evaluate` method returns the valid result or an [`Evaluate::Err`]
/// if an evaluation fails.
///
/// `Expressions` are constructed from [`str`] the most of the time.
/// Use the [`try_into()`] method to create an `Expression` type,
/// the result contain informations about the possible error at conversion time.
///
/// [`Reverse Polish notated`]: https://en.wikipedia.org/wiki/Reverse_Polish_notation
/// [`Evaluate::Err`]: ../evaluate/trait.Evaluate.html#associatedtype.Err
/// [`str`]: https://doc.rust-lang.org/std/str/index.html
/// [`try_into()`]: https://doc.rust-lang.org/std/convert/trait.TryInto.html#tymethod.try_into
#[derive(Debug)]
pub struct Expression<T, O: Evaluate<T>> {
    stack_max: usize,
    expr: Vec<Arithm<T, O>>,
}

impl<T: Copy, O: Evaluate<T> + Copy> Expression<T, O> {
    /// Evaluate the `RPN` expression. Returns the result
    /// or the [`evaluate Error`](../evaluate/trait.Evaluate.html#associatedtype.Err).
    pub fn evaluate(&self) -> Result<T, O::Err> {
        let mut stack = Stack::with_capacity(self.stack_max);
        for arithm in &self.expr {
            match *arithm {
                Arithm::Operand(operand) => stack.push(operand),
                Arithm::Evaluator(evaluator) => evaluator.evaluate(&mut stack)?,
            }
        }
        Ok(stack.pop().unwrap())
    }
}

impl<T, O: Evaluate<T>> Expression<T, O> {
    pub fn from_iter<A, I>(iter: I) -> Result<Expression<T, O>,
                                              ExprResult<<T as TryFromRef<A>>::Err,
                                                         <O as TryFromRef<A>>::Err>>
        where T: TryFromRef<A>,
              O: TryFromRef<A>,
              I: IntoIterator<Item=A>
    {
        let final_expr: Result<Vec<_>, _> = iter.into_iter().map(|token| {
            match TryIntoRef::<T>::try_into_ref(&token) {
                Ok(op) => Ok(Arithm::Operand(op)),
                Err(op_err) => {
                    match TryIntoRef::<O>::try_into_ref(&token) {
                        Ok(eval) => Ok(Arithm::Evaluator(eval)),
                        Err(eval_err) => Err(ExprResult::InvalidToken(op_err, eval_err))
                    }
                }
            }
        }).collect();
        final_expr.and_then(|final_expr| {
            match Expression::check_validity(&final_expr) {
                Ok(_) => Ok(Expression {
                    stack_max: Expression::compute_stack_max(&final_expr),
                    expr: final_expr
                }),
                Err(err) => Err(ExprResult::OperandErr(err)),
            }
        })
    }
}

/// Used to specify the error during the convertion.
#[derive(Debug, PartialEq)]
pub enum ExprResult<A, B> {
    OperandErr(OperandErr),
    InvalidToken(A, B),
}

#[derive(Debug, PartialEq)]
pub enum OperandErr {
    TooManyOperands,
    NotEnoughOperand,
}

impl<T, O: Evaluate<T>> Expression<T, O> {
    fn check_validity(expr: &[Arithm<T, O>]) -> Result<(), OperandErr> {
        // TODO make this more Rusty
        use self::OperandErr::*;
        let mut num_operands: usize = 0;
        for arithm in expr {
            match *arithm {
                Arithm::Operand(_) => num_operands += 1,
                Arithm::Evaluator(ref evaluator) => {
                    let needed = evaluator.operands_needed();
                    num_operands = num_operands.checked_sub(needed).ok_or(NotEnoughOperand)?;
                    num_operands += evaluator.operands_generated();
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

impl<T, O: Evaluate<T>> Expression<T, O> {
    fn compute_stack_max(expr: &[Arithm<T, O>]) -> usize {
        expr.iter().map(|arithm| {
            match *arithm {
                Arithm::Operand(_) => 1,
                Arithm::Evaluator(ref op) => {
                    op.operands_generated() as isize - op.operands_needed() as isize
                }
            }
        }).fold((0, 0isize), |(max, acc_count), count| {
            let acc = (acc_count + count) as usize;
            if acc > max { (acc, acc as isize) } else { (max, acc as isize) }
        }).0
    }
}

impl<T, O> fmt::Display for Expression<T, O>
    where T: fmt::Display,
          O: Evaluate<T> + fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let len = self.expr.len();
        for (i, arithm) in self.expr.iter().enumerate() {
            match *arithm {
                Arithm::Operand(ref operand) => operand.fmt(f)?,
                Arithm::Evaluator(ref evaluator) => evaluator.fmt(f)?,
            }
            if i != len - 1 {
                f.write_str(" ")?
            }
        }
        Ok(())
    }
}
