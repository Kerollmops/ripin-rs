use std::convert::TryFrom;
use std::marker::PhantomData;
use num::Float;
use operate::Operate;
use stack::Stack;
use ::pop_two_operands;

#[derive(Debug, Copy, Clone)]
pub enum FloatOperator<T: Float> {
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

    /// `"sqrt"` will pop `1` operand and push `1`.
    Sqrt(PhantomData<T>),

    /// `"pow"` will pop `1` operand and push `1`.
    Pow(PhantomData<T>),

    /// `"log2"` will pop `1` operand and push `1`.
    Log2(PhantomData<T>),

    /// `"exp"` will pop `1` operand and push `1`.
    Exp(PhantomData<T>),

    /// `"swap"` will pop `2` operands and push `2`.
    Swap(PhantomData<T>),

    /// `"zero"` will pop `0` operand and push `1`.
    Zero(PhantomData<T>),

    /// `"zero"` will pop `0` operand and push `1`.
    One(PhantomData<T>),

    /// `"round"` will pop `1` operand and push `1`.
    Round(PhantomData<T>),
}

#[derive(Debug, PartialEq)]
pub enum FloatOperateErr {
    // TODO add variants
}

impl<T: Float> Operate<T> for FloatOperator<T> {
    type Err = FloatOperateErr;

    fn operands_needed(&self) -> usize {
        use self::FloatOperator::*;
        match *self {
            Add(_) | Sub(_) | Mul(_) | Div(_) | Pow(_) | Rem(_) | Swap(_) => 2,
            Neg(_) | Sqrt(_) | Log2(_) | Round(_) | Exp(_) => 1,
            Zero(_) | One(_) => 0,
        }
    }

    fn operands_generated(&self) -> usize {
        use self::FloatOperator::*;
        match *self {
            Add(_) | Sub(_) | Mul(_) | Div(_) | Rem(_) | Neg(_) | Sqrt(_) | Pow(_) | Log2(_) |
            Exp(_) | Zero(_) | One(_) | Round(_) => 1,
            Swap(_) => 2,
        }
    }

    fn operate(self, stack: &mut Stack<T>) -> Result<(), Self::Err> {
        use self::FloatOperator::*;
        match self {
            Add(_) => {
                let (a, b) = pop_two_operands(stack).unwrap();
                Ok(stack.push(a + b))
            }
            Sub(_) => {
                let (a, b) = pop_two_operands(stack).unwrap();
                Ok(stack.push(a - b))
            }
            Mul(_) => {
                let (a, b) = pop_two_operands(stack).unwrap();
                Ok(stack.push(a * b))
            }
            Div(_) => {
                let (a, b) = pop_two_operands(stack).unwrap();
                Ok(stack.push(a / b))
            }
            Rem(_) => {
                let (a, b) = pop_two_operands(stack).unwrap();
                Ok(stack.push(a % b))
            }
            Neg(_) => {
                let a = stack.pop().unwrap();
                Ok(stack.push(-a))
            }
            Sqrt(_) => {
                let a = stack.pop().unwrap();
                Ok(stack.push(a.sqrt()))
            }
            Pow(_) => {
                let (a, b) = pop_two_operands(stack).unwrap();
                Ok(stack.push(a.powf(b)))
            }
            Log2(_) => {
                let a = stack.pop().unwrap();
                Ok(stack.push(a.log2()))
            }
            Exp(_) => {
                let a = stack.pop().unwrap();
                Ok(stack.push(a.exp()))
            }
            Swap(_) => {
                let (a, b) = pop_two_operands(stack).unwrap();
                stack.push(b);
                stack.push(a);
                Ok(())
            }
            Zero(_) => Ok(stack.push(T::zero())),
            One(_) => Ok(stack.push(T::one())),
            Round(_) => {
                let a = stack.pop().unwrap();
                Ok(stack.push(a.round()))
            }
        }
    }
}

#[derive(Debug)]
pub enum FloatErr<'a> {
    InvalidExpr(&'a str),
}

impl<'a, T: Float> TryFrom<&'a str> for FloatOperator<T> {
    type Err = FloatErr<'a>;
    fn try_from(expr: &'a str) -> Result<Self, Self::Err> {
        use self::FloatOperator::*;
        match expr {
            "+" => Ok(Add(PhantomData::default())),
            "-" => Ok(Sub(PhantomData::default())),
            "*" => Ok(Mul(PhantomData::default())),
            "/" => Ok(Div(PhantomData::default())),
            "%" => Ok(Rem(PhantomData::default())),
            "neg" => Ok(Neg(PhantomData::default())),
            "sqrt" => Ok(Sqrt(PhantomData::default())),
            "pow" => Ok(Pow(PhantomData::default())),
            "log2" => Ok(Log2(PhantomData::default())),
            "exp" => Ok(Exp(PhantomData::default())),
            "swap" => Ok(Swap(PhantomData::default())),
            "zero" => Ok(Zero(PhantomData::default())),
            "one" => Ok(One(PhantomData::default())),
            "round" => Ok(Round(PhantomData::default())),
            _ => Err(FloatErr::InvalidExpr(expr)),
        }
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
}
