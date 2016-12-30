use std::convert::TryFrom;
use std::marker::PhantomData;
use num::Float;
use operate::Operate;
use stack::Stack;
use ::pop_two_operands;

// Add  (+)      pop 2, push 1
// Sub  (-)      pop 2, push 1
// Mul  (*)      pop 2, push 1
// Div  (/)      pop 2, push 1
// Rem  (%)      pop 2, push 1
// Neg  (neg)    pop 1, push 1
// Sqrt (sqrt)   pop 1, push 1
// Pow  (pow)    pop 1, push 1
// Log2 (log2)   pop 1, push 1
// Exp  (exp)    pop 1, push 1
// Swap (swap)   pop 2, push 2
// Zero (zero)   pop 0, push 1
// One  (zero)   pop 0, push 1
// Round (round) pop 1, push 1

#[derive(Debug, Copy, Clone)]
pub enum FloatOperator<T: Float> {
    Add(PhantomData<T>),
    Sub(PhantomData<T>),
    Mul(PhantomData<T>),
    Div(PhantomData<T>),
    Rem(PhantomData<T>),
    Neg(PhantomData<T>),
    Sqrt(PhantomData<T>),
    Pow(PhantomData<T>),
    Log2(PhantomData<T>),
    Exp(PhantomData<T>),
    Swap(PhantomData<T>),
    Zero(PhantomData<T>),
    One(PhantomData<T>),
    Round(PhantomData<T>),
}

impl<T: Float> Operate<T> for FloatOperator<T> {
    type Err = ();

    fn operands_needed(&self) -> usize {
        use self::FloatOperator::*;
        match *self {
              Add(_)
            | Sub(_)
            | Mul(_)
            | Div(_)
            | Pow(_)
            | Rem(_)
            | Swap(_) => 2,
              Neg(_)
            | Sqrt(_)
            | Log2(_)
            | Round(_)
            | Exp(_) => 1,
              Zero(_)
            | One(_) => 0,
        }
    }

    fn operands_generated(&self) -> usize {
        use self::FloatOperator::*;
        match *self {
              Add(_)
            | Sub(_)
            | Mul(_)
            | Div(_)
            | Rem(_)
            | Neg(_)
            | Sqrt(_)
            | Pow(_)
            | Log2(_)
            | Exp(_)
            | Zero(_)
            | One(_)
            | Round(_) => 1,
              Swap(_) => 2,
        }
    }

    fn operate(self, stack: &mut Stack<T>) -> Result<(), Self::Err> {
        use self::FloatOperator::*;
        match self {
            Add(_) => {
                let (a, b) = pop_two_operands(stack).unwrap();
                Ok(stack.push(a + b))
            },
            Sub(_) => {
                let (a, b) = pop_two_operands(stack).unwrap();
                Ok(stack.push(b - a))
            },
            Mul(_) => {
                let (a, b) = pop_two_operands(stack).unwrap();
                Ok(stack.push(a * b))
            },
            Div(_) => {
                let (a, b) = pop_two_operands(stack).unwrap();
                Ok(stack.push(b / a))
            },
            Rem(_) => {
                let (a, b) = pop_two_operands(stack).unwrap();
                Ok(stack.push(b % a))
            },
            Neg(_) => {
                let a = stack.pop().unwrap();
                Ok(stack.push(-a))
            },
            Sqrt(_) => {
                let a = stack.pop().unwrap();
                Ok(stack.push(a.sqrt()))
            },
            Pow(_) => {
                let (a, b) = pop_two_operands(stack).unwrap();
                Ok(stack.push(b.powf(a)))
            },
            Log2(_) => {
                let a = stack.pop().unwrap();
                Ok(stack.push(a.log2()))
            },
            Exp(_) => {
                let a = stack.pop().unwrap();
                Ok(stack.push(a.exp()))
            },
            Swap(_) => {
                let (a, b) = pop_two_operands(stack).unwrap();
                stack.push(a);
                stack.push(b);
                Ok(())
            },
            Zero(_) => Ok(stack.push(T::zero())),
            One(_) => Ok(stack.push(T::one())),
            Round(_) => {
                let a = stack.pop().unwrap();
                Ok(stack.push(a.round()))
            },
        }
    }
}

impl<'a, T: Float> TryFrom<&'a str> for FloatOperator<T> {
    type Err = (); // TODO change this
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
            _ => Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;
    use expression::Expression;
    use operate::FloatOperator;

    #[test]
    #[should_panic]
    fn bad_operator() {
        let _: Expression<f32, FloatOperator<_>> = "3 4 + &".try_into().unwrap();
    }

    #[test]
    #[should_panic]
    fn too_many_operands() {
        let _: Expression<f32, FloatOperator<_>> = "3 3 4 +".try_into().unwrap();
    }

    #[test]
    #[should_panic]
    fn not_enough_operand() {
        let _: Expression<f32, FloatOperator<_>> = "4 +".try_into().unwrap();
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
