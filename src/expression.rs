use std::convert::{From, TryFrom, TryInto};
use std::str::FromStr;
use std::fmt;
use stack::Stack;
use operate::Operate;

/// Used to specify an `Operand` or an `Operator`.
#[derive(Debug, Copy, Clone)]
pub enum Arithm<T, O: Operate<T>> {
    Operand(T),
    Operator(O),
}

/// Interpret a [`Reverse Polish notated`] expression (cf. `3 4 +`).
///
/// `Operate` method returns the valid result or an [`Operate::Err`] if an operation fails.
///
/// `Expressions` are constructed from [`str`] the most of the time.
/// Use the [`try_into()`] method to create an `Expression` type,
/// the result contain informations about the possible error at conversion time.
///
/// [`Reverse Polish notated`]: https://en.wikipedia.org/wiki/Reverse_Polish_notation
/// [`Operate::Err`]: ../operate/trait.Operate.html#associatedtype.Err
/// [`str`]: https://doc.rust-lang.org/std/str/index.html
/// [`try_into()`]: https://doc.rust-lang.org/std/convert/trait.TryInto.html#tymethod.try_into
#[derive(Debug)]
pub struct Expression<T, O: Operate<T>> {
    stack_max: usize,
    expr: Vec<Arithm<T, O>>,
}

impl<T: Copy, O: Operate<T> + Copy> Expression<T, O> {
    /// Evaluate the `RPN` expression. Returns the result or the [`operate Error`].
    ///
    /// [`operate Error`]: ../operate/trait.Operate.html#associatedtype.Err
    pub fn operate(&self) -> Result<T, O::Err> {
        let mut stack = Stack::with_capacity(self.stack_max);
        for arithm in &self.expr {
            match *arithm {
                Arithm::Operand(operand) => stack.push(operand),
                Arithm::Operator(operator) => operator.operate(&mut stack)?,
            }
        }
        Ok(stack.pop().unwrap())
    }
}

/// Used to specify the error during the convertion.
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
    fn check_validity(expr: &[Arithm<T, O>])
        -> Result<(), ExprResult<<T as FromStr>::Err, <O as TryFrom<&'a str>>::Err>> {
        use self::ExprResult::*;
        let mut num_operands: usize = 0;
        for arithm in expr {
            match *arithm {
                Arithm::Operand(_) => num_operands += 1,
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

impl<T, O: Operate<T>> Expression<T, O> {
    fn compute_stack_max(expr: &[Arithm<T, O>]) -> usize {
        expr.iter().map(|arithm| {
            match *arithm {
                Arithm::Operand(_) => 1,
                Arithm::Operator(ref op) => {
                    op.operands_generated() as isize - op.operands_needed() as isize
                }
            }
        }).fold((0, 0isize), |(max, acc_count), count| {
            let acc = (acc_count + count) as usize;
            if acc > max { (acc, acc as isize) } else { (max, acc as isize) }
        }).0
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
        Expression::check_validity(&final_expr)
            .map(|_| {
                Expression {
                    stack_max: Expression::compute_stack_max(&final_expr),
                    expr: final_expr
                }
            })
    }
}

impl<T, O: Operate<T>> From<Vec<Arithm<T, O>>> for Expression<T, O> {
    fn from(expr: Vec<Arithm<T, O>>) -> Self {
        Expression {
            stack_max: Expression::compute_stack_max(&expr),
            expr: expr
        }
    }
}

impl<T, O: Operate<T>> Into<Vec<Arithm<T, O>>> for Expression<T, O> {
    fn into(self) -> Vec<Arithm<T, O>> {
        self.expr
    }
}

impl<T, O> fmt::Display for Expression<T, O>
    where T: fmt::Display,
          O: Operate<T> + fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let len = self.expr.len();
        for (i, arithm) in self.expr.iter().enumerate() {
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
