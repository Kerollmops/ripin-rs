use std::convert::TryFrom;
use std::marker::PhantomData;
use num::{PrimInt, Signed};
use operate::Operate;
use stack::Stack;
use ::pop_two_operands;

#[derive(Debug, Copy, Clone)]
pub enum IntOperator<T: PrimInt + Signed> {
    /// `"+"` will pop `2` operands and push `1`.
    Add(PhantomData<T>),

    /// `"-"` will pop `2` operands and push `1`.
    Sub(PhantomData<T>),

    /// `"*"` will pop `2` operands and push `1`.
    Mul(PhantomData<T>),

    /// `"/"` will pop `2` operands and push `1`.
    Div(PhantomData<T>),

    /// `"%"` will pop `2` operands and push `1`.
    Rem(PhantomData<T>),

    /// `"neg"` will pop `1` operand and push `1`.
    Neg(PhantomData<T>),

    /// `"pow"` will pop `1` operand and push `1`.
    Pow(PhantomData<T>),

    /// `"swap"` will pop `2` operands and push `2`.
    Swap(PhantomData<T>),

    /// `"zero"` will pop `0` operand and push `1`.
    Zero(PhantomData<T>),

    /// `"zero"` will pop `0` operand and push `1`.
    One(PhantomData<T>),
}

#[derive(Debug, PartialEq)]
pub enum IntOperateErr<T> {
    ConvertToU32(T),
    AddOverflow(T, T),
    SubUnderflow(T, T),
    MulOverflow(T, T),
    InvalidDiv(T, T),
    InvalidRem(T, T),
}

impl<T: PrimInt + Signed> Operate<T> for IntOperator<T> {
    type Err = IntOperateErr<T>;

    fn operands_needed(&self) -> usize {
        use self::IntOperator::*;
        match *self {
            Add(_) | Sub(_) | Mul(_) | Div(_) | Pow(_) | Rem(_) | Swap(_) => 2,
            Neg(_) => 1,
            Zero(_) | One(_) => 0,
        }
    }

    fn operands_generated(&self) -> usize {
        use self::IntOperator::*;
        match *self {
            Add(_) | Sub(_) | Mul(_) | Div(_) | Rem(_) | Neg(_) | Pow(_) | Zero(_) | One(_) => 1,
            Swap(_) => 2,
        }
    }

    fn operate(self, stack: &mut Stack<T>) -> Result<(), Self::Err> {
        use self::IntOperator::*;
        use self::IntOperateErr::*;
        match self {
            Add(_) => {
                let (a, b) = pop_two_operands(stack).unwrap();
                let c = a.checked_add(&b).ok_or(AddOverflow(a, b))?;
                Ok(stack.push(c))
            }
            Sub(_) => {
                let (a, b) = pop_two_operands(stack).unwrap();
                let c = b.checked_sub(&a).ok_or(SubUnderflow(b, a))?;
                Ok(stack.push(c))
            }
            Mul(_) => {
                let (a, b) = pop_two_operands(stack).unwrap();
                let c = a.checked_mul(&b).ok_or(MulOverflow(a, b))?;
                Ok(stack.push(c))
            }
            Div(_) => {
                let (a, b) = pop_two_operands(stack).unwrap();
                let c = b.checked_div(&a).ok_or(InvalidDiv(b, a))?;
                Ok(stack.push(c))
            }
            Rem(_) => {
                let (a, b) = pop_two_operands(stack).unwrap();
                if a == T::zero() { return Err(InvalidRem(b, a)) }
                Ok(stack.push(b % a))
            }
            Neg(_) => {
                let a = stack.pop().unwrap();
                Ok(stack.push(-a))
            }
            Pow(_) => {
                let (a, b) = pop_two_operands(stack).unwrap();
                let exp = a.to_u32().ok_or(ConvertToU32(a))?; // TODO check overflow !
                Ok(stack.push(b.pow(exp)))
            }
            Swap(_) => {
                let (a, b) = pop_two_operands(stack).unwrap();
                stack.push(a);
                stack.push(b);
                Ok(())
            }
            Zero(_) => Ok(stack.push(T::zero())),
            One(_) => Ok(stack.push(T::one())),
        }
    }
}

#[derive(Debug)]
pub enum IntErr<'a> {
    InvalidExpr(&'a str),
}

impl<'a, T: PrimInt + Signed> TryFrom<&'a str> for IntOperator<T> {
    type Err = IntErr<'a>;
    fn try_from(expr: &'a str) -> Result<Self, Self::Err> {
        use self::IntOperator::*;
        match expr {
            "+" => Ok(Add(PhantomData::default())),
            "-" => Ok(Sub(PhantomData::default())),
            "*" => Ok(Mul(PhantomData::default())),
            "/" => Ok(Div(PhantomData::default())),
            "%" => Ok(Rem(PhantomData::default())),
            "neg" => Ok(Neg(PhantomData::default())),
            "pow" => Ok(Pow(PhantomData::default())),
            "swap" => Ok(Swap(PhantomData::default())),
            "zero" => Ok(Zero(PhantomData::default())),
            "one" => Ok(One(PhantomData::default())),
            _ => Err(IntErr::InvalidExpr(expr)),
        }
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
        assert_eq!(expr.operate(), Err(IntOperateErr::AddOverflow(20, 125)));
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
        assert_eq!(expr.operate(), Err(IntOperateErr::MulOverflow(20, 30)));
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
    #[should_panic]
    fn overflowing_power() {
        let expr: Expression<i8, IntOperator<_>> = "3 10 pow".try_into().unwrap();
        let _ = expr.operate();
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
}