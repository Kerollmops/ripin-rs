use std::fmt;
use stack::Stack;
use evaluate::Evaluate;
use variable::{GetVariable, DummyVariables};
use convert_ref::{TryFromRef, TryIntoRef};

/// Used to specify an `Operand` or an `Evaluator`.
#[derive(Debug, Copy, Clone)]
pub enum Arithm<T, V, E: Evaluate<T>> {
    Operand(T),
    Variable(V),
    Evaluator(E),
}

/// Interpret a [`Reverse Polish notated`] expression (cf. `3 4 +`).
///
/// `Evaluate` method returns the valid result or an [`Evaluate::Err`]
/// if the evaluation fails.
///
/// `Expressions` are constructed from [`str`] the most of the time.
/// Use the [`try_into_ref()`] method to create an `Expression` type,
/// the result contain informations about the possible error at conversion time.
///
/// [`Reverse Polish notated`]: https://en.wikipedia.org/wiki/Reverse_Polish_notation
/// [`Evaluate::Err`]: ../evaluate/trait.Evaluate.html#associatedtype.Err
/// [`str`]: https://doc.rust-lang.org/std/str/index.html
/// [`try_into_ref()`]: ../convert_ref/trait.TryIntoRef.html
#[derive(Debug)]
pub struct Expression<T, V, E: Evaluate<T>> {
    max_stack: usize,
    expr: Vec<Arithm<T, V, E>>,
}

impl<T: Copy, V: Copy, E: Evaluate<T> + Copy> Expression<T, V, E> {
    /// Evaluate `RPN` expressions. Returns the result
    /// or the [`evaluate Error`](../evaluate/trait.Evaluate.html#associatedtype.Err).
    pub fn evaluate(&self) -> Result<T, E::Err> where (): From<V> {
        self.evaluate_with_variables(&DummyVariables::default())
    }

    /// Evaluate `RPN` expressions containing variables. Returns the result
    /// or the [`evaluate Error`](../evaluate/trait.Evaluate.html#associatedtype.Err).
    ///
    /// # Panics
    /// Panics if a variables doesn't exists in the variable container.
    pub fn evaluate_with_variables<I, C>(&self, variables: &C) -> Result<T, E::Err>
        where V: Into<I>,
              C: GetVariable<I, Output=T>
    {
        let mut stack = Stack::with_capacity(self.max_stack);
        for arithm in &self.expr {
            match *arithm {
                Arithm::Operand(operand) => stack.push(operand),
                Arithm::Variable(var) => {
                    let var = variables.get_variable(var.into()).expect("TODO Variable not found!");
                    stack.push(*var)
                },
                Arithm::Evaluator(evaluator) => evaluator.evaluate(&mut stack)?,
            }
        }
        Ok(stack.pop().unwrap())
    }
}

impl<T, V, E: Evaluate<T>> Expression<T, V, E> {
    pub fn from_iter<A, I>(iter: I) -> Result<Expression<T, V, E>,
                                              ExprResult<<E as TryFromRef<A>>::Err,
                                                         <V as TryFromRef<A>>::Err,
                                                         <T as TryFromRef<A>>::Err>>
        where T: TryFromRef<A>,
              V: TryFromRef<A>,
              E: TryFromRef<A>,
              I: IntoIterator<Item=A>
    {
        let final_expr: Result<Vec<_>, _> = iter.into_iter().map(|token| {
            match TryIntoRef::<E>::try_into_ref(&token) {
                Ok(eval) => Ok(Arithm::Evaluator(eval)),
                Err(eval_err) => {
                    match TryIntoRef::<V>::try_into_ref(&token) {
                        Ok(var) => Ok(Arithm::Variable(var)),
                        Err(var_err) => {
                            match TryIntoRef::<T>::try_into_ref(&token) {
                                Ok(op) => Ok(Arithm::Operand(op)),
                                Err(op_err) => Err(ExprResult::InvalidToken {
                                    evaluator: eval_err,
                                    variable: var_err,
                                    operand: op_err,
                                })
                            }
                        }
                    }
                }
            }
        }).collect();
        final_expr.and_then(|final_expr| {
            match Expression::check_validity(&final_expr) {
                Ok(_) => Ok(Expression {
                    max_stack: Expression::compute_stack_max(&final_expr),
                    expr: final_expr
                }),
                Err(err) => Err(ExprResult::OperandErr(err)),
            }
        })
    }
}

/// Used to specify the error during the conversion.
#[derive(Debug, PartialEq)]
pub enum ExprResult<A, B, C> {
    OperandErr(OperandErr),
    InvalidToken {
        evaluator: A,
        variable: B,
        operand: C
    },
}

/// Used to specify an error related to wrong number of operands in expression.
#[derive(Debug, PartialEq)]
pub enum OperandErr {
    TooManyOperands,
    NotEnoughOperand,
}

impl<T, V, E: Evaluate<T>> Expression<T, V, E> {
    fn check_validity(expr: &[Arithm<T, V, E>]) -> Result<(), OperandErr> {
        // TODO https://doc.rust-lang.org/1.2.0/std/result/fn.fold.html
        use self::OperandErr::*;
        let mut num_operands: usize = 0;
        for arithm in expr {
            match *arithm {
                Arithm::Operand(_) | Arithm::Variable(_) => num_operands += 1,
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

impl<T, V, E: Evaluate<T>> Expression<T, V, E> {
    fn compute_stack_max(expr: &[Arithm<T, V, E>]) -> usize {
        expr.iter().map(|arithm| {
            match *arithm {
                Arithm::Operand(_) | Arithm::Variable(_) => 1,
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

impl<T, V, E> fmt::Display for Expression<T, V, E>
    where T: fmt::Display,
          V: fmt::Display,
          E: fmt::Display + Evaluate<T>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let len = self.expr.len();
        for (i, arithm) in self.expr.iter().enumerate() {
            match *arithm {
                Arithm::Operand(ref operand) => operand.fmt(f)?,
                Arithm::Variable(ref variable) => variable.fmt(f)?,
                Arithm::Evaluator(ref evaluator) => evaluator.fmt(f)?,
            }
            if i != len - 1 {
                f.write_str(" ")?
            }
        }
        Ok(())
    }
}
