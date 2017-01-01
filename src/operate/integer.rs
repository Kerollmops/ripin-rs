use std::convert::TryFrom;
use std::marker::PhantomData;
use std::fmt;
use num::{PrimInt, Signed, checked_pow};
use operate::Operate;
use stack::Stack;
use ::pop_two_operands;

/// Basic Signed Integer Operator for any type that implement [`PrimInt`] and [`Signed`] Traits.
///
/// [`PrimInt`]: http://rust-num.github.io/num/num/trait.PrimInt.html
/// [`Signed`]: http://rust-num.github.io/num/num/trait.Signed.html
#[derive(Debug, Copy, Clone)]
pub enum IntOperator<T: PrimInt + Signed> {
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
    /// `"pow"` will pop `2` operands and push `1`.
    Pow,
    /// `"swap"` will pop `2` operands and push `2`.
    Swap,
    /// `"zero"` will pop `0` operand and push `1`.
    Zero,
    /// `"zero"` will pop `0` operand and push `1`.
    One,
    #[doc(hidden)]
    _Phantom(PhantomData<T>)
}

/// Type returned when an error occurs on signed integer operation.
#[derive(Debug, PartialEq)]
pub enum IntOperateErr<T> {
    ConvertToU32(T),
    AddOverflow(T, T),
    SubUnderflow(T, T),
    MulOverflow(T, T),
    PowOverflow(T, usize),
    InvalidDiv(T, T),
    InvalidRem(T, T),
}

impl<T: PrimInt + Signed> Operate<T> for IntOperator<T> {
    type Err = IntOperateErr<T>;

    fn operands_needed(&self) -> usize {
        use self::IntOperator::*;
        match *self {
            Add | Sub | Mul | Div | Pow | Rem | Swap => 2,
            Neg => 1,
            Zero | One => 0,
            _Phantom(_) => unreachable!()
        }
    }

    fn operands_generated(&self) -> usize {
        use self::IntOperator::*;
        match *self {
            Add | Sub | Mul | Div | Rem | Neg | Pow | Zero | One => 1,
            Swap => 2,
            _Phantom(_) => unreachable!()
        }
    }

    fn operate(self, stack: &mut Stack<T>) -> Result<(), Self::Err> {
        use self::IntOperator::*;
        use self::IntOperateErr::*;
        match self {
            Add => {
                let (a, b) = pop_two_operands(stack).unwrap();
                let c = a.checked_add(&b).ok_or(AddOverflow(a, b))?;
                Ok(stack.push(c))
            }
            Sub => {
                let (a, b) = pop_two_operands(stack).unwrap();
                let c = a.checked_sub(&b).ok_or(SubUnderflow(a, b))?;
                Ok(stack.push(c))
            }
            Mul => {
                let (a, b) = pop_two_operands(stack).unwrap();
                let c = a.checked_mul(&b).ok_or(MulOverflow(a, b))?;
                Ok(stack.push(c))
            }
            Div => {
                let (a, b) = pop_two_operands(stack).unwrap();
                let c = a.checked_div(&b).ok_or(InvalidDiv(a, b))?;
                Ok(stack.push(c))
            }
            Rem => {
                let (a, b) = pop_two_operands(stack).unwrap();
                if b == T::zero() {
                    Err(InvalidRem(a, b))
                }
                else {
                    Ok(stack.push(a % b))
                }
            }
            Neg => {
                let a = stack.pop().unwrap();
                Ok(stack.push(-a))
            }
            Pow => {
                let (a, b) = pop_two_operands(stack).unwrap();
                let b = b.to_usize().ok_or(ConvertToU32(b))?;
                let pow = checked_pow(a, b).ok_or(PowOverflow(a, b))?;
                Ok(stack.push(pow))
            }
            Swap => {
                let (a, b) = pop_two_operands(stack).unwrap();
                stack.push(b);
                stack.push(a);
                Ok(())
            }
            Zero => Ok(stack.push(T::zero())),
            One => Ok(stack.push(T::one())),
            _Phantom(_) => unreachable!()
        }
    }
}

/// Type returned when a conversion cannot be performed.
#[derive(Debug)]
pub enum IntErr<'a> { // TODO change name
    InvalidExpr(&'a str),
}

impl<'a, T: PrimInt + Signed> TryFrom<&'a str> for IntOperator<T> {
    type Err = IntErr<'a>;
    fn try_from(expr: &'a str) -> Result<Self, Self::Err> {
        use self::IntOperator::*;
        match expr {
            "+" => Ok(Add),
            "-" => Ok(Sub),
            "*" => Ok(Mul),
            "/" => Ok(Div),
            "%" => Ok(Rem),
            "neg" => Ok(Neg),
            "pow" => Ok(Pow),
            "swap" => Ok(Swap),
            "zero" => Ok(Zero),
            "one" => Ok(One),
            _ => Err(IntErr::InvalidExpr(expr)),
        }
    }
}

impl<T: PrimInt + Signed> fmt::Display for IntOperator<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::IntOperator::*;
        let name = match *self {
            Add => "+",
            Sub => "-",
            Mul => "*",
            Div => "/",
            Rem => "%",
            Neg => "neg",
            Pow => "pow",
            Swap => "swap",
            Zero => "zero",
            One => "one",
            _Phantom(_) => unreachable!()
        };
        f.write_str(name)
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;
    use expression::{Expression, ExprResult};
    use operate::{IntOperator, IntErr, IntOperateErr};

    #[test]
    fn bad_operator() {
        let res: Result<Expression<i32, IntOperator<_>>, _> = "3 4 + &".try_into();
        match res {
            Err(ExprResult::InvalidToken(_, IntErr::InvalidExpr("&"))) => (),
            _ => panic!(res),
        }
    }

    #[test]
    fn too_many_operands() {
        let res: Result<Expression<i32, IntOperator<_>>, _> = "3 3 4 +".try_into();
        match res {
            Err(ExprResult::TooManyOperands) => (),
            _ => panic!(res),
        }
    }

    #[test]
    fn not_enough_operand() {
        let res: Result<Expression<i32, IntOperator<_>>, _> = "4 +".try_into();
        match res {
            Err(ExprResult::NotEnoughOperand) => (),
            _ => panic!(res),
        }
    }

    #[test]
    fn simple_addition() {
        let expr: Expression<i32, IntOperator<_>> = "3 4 +".try_into().unwrap();
        assert_eq!(expr.operate(), Ok(7));
    }

    #[test]
    fn overflowing_addition() {
        let expr: Expression<i8, IntOperator<_>> = "125 20 +".try_into().unwrap();
        assert_eq!(expr.operate(), Err(IntOperateErr::AddOverflow(125, 20)));
    }

    #[test]
    fn simple_substraction() {
        let expr: Expression<i32, IntOperator<_>> = "4 3 -".try_into().unwrap();
        assert_eq!(expr.operate(), Ok(1));
    }

    #[test]
    fn underflowing_substraction() {
        let expr: Expression<i8, IntOperator<_>> = "-120 30 -".try_into().unwrap();
        assert_eq!(expr.operate(), Err(IntOperateErr::SubUnderflow(-120, 30)));
    }

    #[test]
    fn simple_multiplication() {
        let expr: Expression<i32, IntOperator<_>> = "3 4 *".try_into().unwrap();
        assert_eq!(expr.operate(), Ok(12));
    }

    #[test]
    fn overflowing_multiplication() {
        let expr: Expression<i8, IntOperator<_>> = "30 20 *".try_into().unwrap();
        assert_eq!(expr.operate(), Err(IntOperateErr::MulOverflow(30, 20)));
    }

    #[test]
    fn simple_division() {
        let expr: Expression<i32, IntOperator<_>> = "9 3 /".try_into().unwrap();
        assert_eq!(expr.operate(), Ok(3));
    }

    #[test]
    fn invalid_division() {
        let expr: Expression<i8, IntOperator<_>> = "9 0 /".try_into().unwrap();
        assert_eq!(expr.operate(), Err(IntOperateErr::InvalidDiv(9, 0)));
    }

    #[test]
    fn simple_remaining() {
        let expr: Expression<i32, IntOperator<_>> = "9 3 %".try_into().unwrap();
        assert_eq!(expr.operate(), Ok(0));
    }

    #[test]
    fn invalid_remaining() {
        let expr: Expression<i32, IntOperator<_>> = "9 0 %".try_into().unwrap();
        assert_eq!(expr.operate(), Err(IntOperateErr::InvalidRem(9, 0)));
    }

    #[test]
    fn simple_negation() {
        let expr: Expression<i32, IntOperator<_>> = "9 neg".try_into().unwrap();
        assert_eq!(expr.operate(), Ok(-9));
    }

    #[test]
    fn simple_power() {
        let expr: Expression<i32, IntOperator<_>> = "3 4 pow".try_into().unwrap();
        assert_eq!(expr.operate(), Ok(81));
    }

    #[test]
    fn overflowing_power() {
        let expr: Expression<i8, IntOperator<_>> = "3 10 pow".try_into().unwrap();
        assert_eq!(expr.operate(), Err(IntOperateErr::PowOverflow(3, 10)));
    }

    #[test]
    fn invalid_exp_power() {
        let expr: Expression<i8, IntOperator<_>> = "3 -10 pow".try_into().unwrap();
        assert_eq!(expr.operate(), Err(IntOperateErr::ConvertToU32(-10)));
    }

    #[test]
    fn simple_swap() {
        let expr: Expression<i32, IntOperator<_>> = "2 4 swap /".try_into().unwrap();
        assert_eq!(expr.operate(), Ok(2));
    }

    #[test]
    fn simple_zero() {
        let expr: Expression<i32, IntOperator<_>> = "zero".try_into().unwrap();
        assert_eq!(expr.operate(), Ok(0));
    }

    #[test]
    fn simple_one() {
        let expr: Expression<i32, IntOperator<_>> = "one".try_into().unwrap();
        assert_eq!(expr.operate(), Ok(1));
    }

    #[test]
    fn to_string() {
        let expr_str = "3 3 + neg neg 4 +";
        let expr: Expression<i32, IntOperator<_>> = expr_str.try_into().unwrap();
        assert_eq!(&expr.to_string(), expr_str);
    }
}
