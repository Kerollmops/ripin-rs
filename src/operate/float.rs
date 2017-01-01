use std::convert::TryFrom;
use std::marker::PhantomData;
use std::fmt;
use num::Float;
use operate::Operate;
use stack::Stack;
use ::pop_two_operands;

/// Basic Float Operator for any type that implement the [`Float`] Trait.
///
/// [`Float`]: http://rust-num.github.io/num/num/trait.Float.html
#[derive(Debug, Copy, Clone)]
pub enum FloatOperator<T: Float> {
    /// `"+"` will pop `2` operands and push `1`.
    Add,
    /// `"-"` will pop `2` operands and push `1`.
    Sub,
    /// `"*"` will pop `2` operands and push `1`.
    Mul,
    /// `"/"` will pop `2` operands and push `1`.
    Div,
    /// `"%"` will pop `2` operands and push `1`.
    Rem,
    /// `"neg"` will pop `1` operand and push `1`.
    Neg,
    /// `"sqrt"` will pop `1` operand and push `1`.
    Sqrt,
    /// `"pow"` will pop `2` operands and push `1`.
    Pow,
    /// `"log2"` will pop `1` operand and push `1`.
    Log2,
    /// `"exp"` will pop `1` operand and push `1`.
    Exp,
    /// `"swap"` will pop `2` operands and push `2`.
    Swap,
    /// `"zero"` will pop `0` operand and push `1`.
    Zero,
    /// `"zero"` will pop `0` operand and push `1`.
    One,
    /// `"round"` will pop `1` operand and push `1`.
    Round,
    #[doc(hidden)]
    _Phantom(PhantomData<T>)
}

/// Type returned when an error occurs on float operation.
#[derive(Debug, PartialEq)]
pub enum FloatOperateErr {
    // TODO add variants
}

impl<T: Float> Operate<T> for FloatOperator<T> {
    type Err = FloatOperateErr;

    fn operands_needed(&self) -> usize {
        use self::FloatOperator::*;
        match *self {
            Add | Sub | Mul | Div | Pow | Rem | Swap => 2,
            Neg | Sqrt | Log2 | Round | Exp => 1,
            Zero | One => 0,
            _Phantom(_) => unreachable!()
        }
    }

    fn operands_generated(&self) -> usize {
        use self::FloatOperator::*;
        match *self {
            Add | Sub | Mul | Div | Rem | Neg | Sqrt | Pow | Log2 |
            Exp | Zero | One | Round => 1,
            Swap => 2,
            _Phantom(_) => unreachable!()
        }
    }

    fn operate(self, stack: &mut Stack<T>) -> Result<(), Self::Err> {
        use self::FloatOperator::*;
        match self {
            Add => {
                let (a, b) = pop_two_operands(stack).unwrap();
                Ok(stack.push(a + b))
            }
            Sub => {
                let (a, b) = pop_two_operands(stack).unwrap();
                Ok(stack.push(a - b))
            }
            Mul => {
                let (a, b) = pop_two_operands(stack).unwrap();
                Ok(stack.push(a * b))
            }
            Div => {
                let (a, b) = pop_two_operands(stack).unwrap();
                Ok(stack.push(a / b))
            }
            Rem => {
                let (a, b) = pop_two_operands(stack).unwrap();
                Ok(stack.push(a % b))
            }
            Neg => {
                let a = stack.pop().unwrap();
                Ok(stack.push(-a))
            }
            Sqrt => {
                let a = stack.pop().unwrap();
                Ok(stack.push(a.sqrt()))
            }
            Pow => {
                let (a, b) = pop_two_operands(stack).unwrap();
                Ok(stack.push(a.powf(b)))
            }
            Log2 => {
                let a = stack.pop().unwrap();
                Ok(stack.push(a.log2()))
            }
            Exp => {
                let a = stack.pop().unwrap();
                Ok(stack.push(a.exp()))
            }
            Swap => {
                let (a, b) = pop_two_operands(stack).unwrap();
                stack.push(b);
                stack.push(a);
                Ok(())
            }
            Zero => Ok(stack.push(T::zero())),
            One => Ok(stack.push(T::one())),
            Round => {
                let a = stack.pop().unwrap();
                Ok(stack.push(a.round()))
            },
            _Phantom(_) => unreachable!()
        }
    }
}

/// Type returned when a conversion cannot be performed.
#[derive(Debug)]
pub enum FloatErr<'a> { // TODO change name
    InvalidExpr(&'a str),
}

impl<'a, T: Float> TryFrom<&'a str> for FloatOperator<T> {
    type Err = FloatErr<'a>;
    fn try_from(expr: &'a str) -> Result<Self, Self::Err> {
        use self::FloatOperator::*;
        match expr {
            "+" => Ok(Add),
            "-" => Ok(Sub),
            "*" => Ok(Mul),
            "/" => Ok(Div),
            "%" => Ok(Rem),
            "neg" => Ok(Neg),
            "sqrt" => Ok(Sqrt),
            "pow" => Ok(Pow),
            "log2" => Ok(Log2),
            "exp" => Ok(Exp),
            "swap" => Ok(Swap),
            "zero" => Ok(Zero),
            "one" => Ok(One),
            "round" => Ok(Round),
            _ => Err(FloatErr::InvalidExpr(expr)),
        }
    }
}

impl<T: Float> fmt::Display for FloatOperator<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::FloatOperator::*;
        let name = match *self {
            Add => "+",
            Sub => "-",
            Mul => "*",
            Div => "/",
            Rem => "%",
            Neg => "neg",
            Sqrt => "sqrt",
            Pow => "pow",
            Log2 => "log2",
            Exp => "exp",
            Swap => "swap",
            Zero => "zero",
            One => "one",
            Round => "round",
            _Phantom(_) => unreachable!()
        };
        f.write_str(name)
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;
    use expression::{Expression, ExprResult};
    use operate::{FloatOperator, FloatErr};

    #[test]
    fn bad_operator() {
        let res: Result<Expression<f32, FloatOperator<_>>, _> = "3 4 + &".try_into();
        match res {
            Err(ExprResult::InvalidToken(_, FloatErr::InvalidExpr("&"))) => (),
            _ => panic!(res),
        }
    }

    #[test]
    fn too_many_operands() {
        let res: Result<Expression<f32, FloatOperator<_>>, _> = "3 3 4 +".try_into();
        match res {
            Err(ExprResult::TooManyOperands) => (),
            _ => panic!(res),
        }
    }

    #[test]
    fn not_enough_operand() {
        let res: Result<Expression<f32, FloatOperator<_>>, _> = "4 +".try_into();
        match res {
            Err(ExprResult::NotEnoughOperand) => (),
            _ => panic!(res),
        }
    }

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
    fn simple_division() {
        let expr: Expression<f32, FloatOperator<_>> = "9 3 /".try_into().unwrap();
        assert_eq!(expr.operate(), Ok(3.0));
    }

    #[test]
    fn simple_division_by_zero() {
        use std::f32;
        let expr: Expression<f32, FloatOperator<_>> = "9 0 /".try_into().unwrap();
        assert_eq!(expr.operate(), Ok(f32::INFINITY));
    }

    #[test]
    fn simple_nan() {
        let expr: Expression<f32, FloatOperator<_>> = "0 0 /".try_into().unwrap();
        assert!(expr.operate().unwrap().is_nan());
    }

    #[test]
    fn simple_remaining() {
        let expr: Expression<f32, FloatOperator<_>> = "9 3 %".try_into().unwrap();
        assert_eq!(expr.operate(), Ok(0.0));
    }

    #[test]
    fn simple_negation() {
        let expr: Expression<f32, FloatOperator<_>> = "9 neg".try_into().unwrap();
        assert_eq!(expr.operate(), Ok(-9.0));
    }

    #[test]
    fn simple_square_root() {
        let expr: Expression<f32, FloatOperator<_>> = "9 sqrt".try_into().unwrap();
        assert_eq!(expr.operate(), Ok(3.0));
    }

    #[test]
    fn simple_power() {
        let expr: Expression<f32, FloatOperator<_>> = "3 4 pow".try_into().unwrap();
        assert_eq!(expr.operate(), Ok(81.0));
    }

    #[test]
    fn simple_logarithm_2() {
        let expr: Expression<f32, FloatOperator<_>> = "4 log2".try_into().unwrap();
        assert_eq!(expr.operate(), Ok(2.0));
    }

    #[test]
    fn simple_exponential() {
        let expr: Expression<f32, FloatOperator<_>> = "0 exp".try_into().unwrap();
        assert_eq!(expr.operate(), Ok(1.0));
    }

    #[test]
    fn simple_swap() {
        let expr: Expression<f32, FloatOperator<_>> = "2 4 swap /".try_into().unwrap();
        assert_eq!(expr.operate(), Ok(2.0));
    }

    #[test]
    fn simple_zero() {
        let expr: Expression<f32, FloatOperator<_>> = "zero".try_into().unwrap();
        assert_eq!(expr.operate(), Ok(0.0));
    }

    #[test]
    fn simple_one() {
        let expr: Expression<f32, FloatOperator<_>> = "one".try_into().unwrap();
        assert_eq!(expr.operate(), Ok(1.0));
    }

    #[test]
    fn simple_round() {
        let expr: Expression<f32, FloatOperator<_>> = "3.3 round".try_into().unwrap();
        assert_eq!(expr.operate(), Ok(3.0));
    }

    #[test]
    fn to_string() {
        let expr_str = "3.3 3 + round neg 4 +";
        let expr: Expression<f32, FloatOperator<_>> = expr_str.try_into().unwrap();
        assert_eq!(&expr.to_string(), expr_str);
    }
}
