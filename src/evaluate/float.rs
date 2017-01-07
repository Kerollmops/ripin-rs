use std::marker::PhantomData;
use std::fmt;
use num::Float;
use evaluate::Evaluate;
use stack::Stack;
use ::pop_two_operands;
use convert_ref::TryFromRef;

/// Basic Float Evaluator for any type that implement the [`Float`] Trait.
///
/// [`Float`]: http://rust-num.github.io/num/num/trait.Float.html
#[derive(Debug, Copy, Clone)]
pub enum FloatEvaluator<T: Float> {
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
pub enum FloatEvaluateErr {
    // TODO add variants
}

impl<T: Float> Evaluate<T> for FloatEvaluator<T> {
    type Err = FloatEvaluateErr;

    fn operands_needed(&self) -> usize {
        use self::FloatEvaluator::*;
        match *self {
            Add | Sub | Mul | Div | Pow | Rem | Swap => 2,
            Neg | Sqrt | Log2 | Round | Exp => 1,
            Zero | One => 0,
            _Phantom(_) => unreachable!()
        }
    }

    fn operands_generated(&self) -> usize {
        use self::FloatEvaluator::*;
        match *self {
            Add | Sub | Mul | Div | Rem | Neg | Sqrt | Pow | Log2 |
            Exp | Zero | One | Round => 1,
            Swap => 2,
            _Phantom(_) => unreachable!()
        }
    }

    fn evaluate(self, stack: &mut Stack<T>) -> Result<(), Self::Err> {
        use self::FloatEvaluator::*;
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

impl<'a, T: Float> TryFromRef<&'a str> for FloatEvaluator<T> {
    type Err = FloatErr<'a>;
    fn try_from_ref(expr: &&'a str) -> Result<Self, Self::Err> {
        use self::FloatEvaluator::*;
        match *expr {
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

impl<T: Float> fmt::Display for FloatEvaluator<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::FloatEvaluator::*;
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
    use expression::{ExprResult, OperandErr};
    use evaluate::{FloatErr, FloatExpr, VariableFloatExpr};

    #[test]
    fn bad_operator() {
        let expr = "3 4 + &";
        let tokens = expr.split_whitespace();
        let res = FloatExpr::<f32>::from_iter(tokens);
        match res {
            Err(ExprResult::InvalidToken { evaluator: FloatErr::InvalidExpr("&"), .. } ) => (),
            _ => panic!(res),
        }
    }

    #[test]
    fn too_many_operands() {
        let expr = "3 3 4 +";
        let tokens = expr.split_whitespace();
        let res = FloatExpr::<f32>::from_iter(tokens);
        match res {
            Err(ExprResult::OperandErr(OperandErr::TooManyOperands)) => (),
            _ => panic!(res),
        }
    }

    #[test]
    fn not_enough_operand() {
        let expr = "4 +";
        let tokens = expr.split_whitespace();
        let res = FloatExpr::<f32>::from_iter(tokens);
        match res {
            Err(ExprResult::OperandErr(OperandErr::NotEnoughOperand)) => (),
            _ => panic!(res),
        }
    }

    #[test]
    fn simple_addition() {
        let expr_str = "3 4 +";
        let tokens = expr_str.split_whitespace();
        let expr = FloatExpr::<f32>::from_iter(tokens).unwrap();
        assert_eq!(expr.evaluate(), Ok(7.0));
    }

    #[test]
    fn simple_substraction() {
        let expr_str = "4 3 -";
        let tokens = expr_str.split_whitespace();
        let expr = FloatExpr::<f32>::from_iter(tokens).unwrap();
        assert_eq!(expr.evaluate(), Ok(1.0));
    }

    #[test]
    fn simple_multiplication() {
        let expr_str = "3 4 *";
        let tokens = expr_str.split_whitespace();
        let expr = FloatExpr::<f32>::from_iter(tokens).unwrap();
        assert_eq!(expr.evaluate(), Ok(12.0));
    }

    #[test]
    fn simple_division() {
        let expr_str = "9 3 /";
        let tokens = expr_str.split_whitespace();
        let expr = FloatExpr::<f32>::from_iter(tokens).unwrap();
        assert_eq!(expr.evaluate(), Ok(3.0));
    }

    #[test]
    fn simple_division_by_zero() {
        use std::f32;
        let expr_str = "9 0 /";
        let tokens = expr_str.split_whitespace();
        let expr = FloatExpr::<f32>::from_iter(tokens).unwrap();
        assert_eq!(expr.evaluate(), Ok(f32::INFINITY));
    }

    #[test]
    fn simple_nan() {
        let expr_str = "0 0 /";
        let tokens = expr_str.split_whitespace();
        let expr = FloatExpr::<f32>::from_iter(tokens).unwrap();
        assert!(expr.evaluate().unwrap().is_nan());
    }

    #[test]
    fn simple_remaining() {
        let expr_str = "9 3 %";
        let tokens = expr_str.split_whitespace();
        let expr = FloatExpr::<f32>::from_iter(tokens).unwrap();
        assert_eq!(expr.evaluate(), Ok(0.0));
    }

    #[test]
    fn simple_negation() {
        let expr_str = "9 neg";
        let tokens = expr_str.split_whitespace();
        let expr = FloatExpr::<f32>::from_iter(tokens).unwrap();
        assert_eq!(expr.evaluate(), Ok(-9.0));
    }

    #[test]
    fn simple_square_root() {
        let expr_str = "9 sqrt";
        let tokens = expr_str.split_whitespace();
        let expr = FloatExpr::<f32>::from_iter(tokens).unwrap();
        assert_eq!(expr.evaluate(), Ok(3.0));
    }

    #[test]
    fn simple_power() {
        let expr_str = "3 4 pow";
        let tokens = expr_str.split_whitespace();
        let expr = FloatExpr::<f32>::from_iter(tokens).unwrap();
        assert_eq!(expr.evaluate(), Ok(81.0));
    }

    #[test]
    fn simple_logarithm_2() {
        let expr_str = "4 log2";
        let tokens = expr_str.split_whitespace();
        let expr = FloatExpr::<f32>::from_iter(tokens).unwrap();
        assert_eq!(expr.evaluate(), Ok(2.0));
    }

    #[test]
    fn simple_exponential() {
        let expr_str = "0 exp";
        let tokens = expr_str.split_whitespace();
        let expr = FloatExpr::<f32>::from_iter(tokens).unwrap();
        assert_eq!(expr.evaluate(), Ok(1.0));
    }

    #[test]
    fn simple_swap() {
        let expr_str = "2 4 swap /";
        let tokens = expr_str.split_whitespace();
        let expr = FloatExpr::<f32>::from_iter(tokens).unwrap();
        assert_eq!(expr.evaluate(), Ok(2.0));
    }

    #[test]
    fn simple_zero() {
        let expr_str = "zero";
        let tokens = expr_str.split_whitespace();
        let expr = FloatExpr::<f32>::from_iter(tokens).unwrap();
        assert_eq!(expr.evaluate(), Ok(0.0));
    }

    #[test]
    fn simple_one() {
        let expr_str = "one";
        let tokens = expr_str.split_whitespace();
        let expr = FloatExpr::<f32>::from_iter(tokens).unwrap();
        assert_eq!(expr.evaluate(), Ok(1.0));
    }

    #[test]
    fn simple_round() {
        let expr_str = "3.3 round";
        let tokens = expr_str.split_whitespace();
        let expr = FloatExpr::<f32>::from_iter(tokens).unwrap();
        assert_eq!(expr.evaluate(), Ok(3.0));
    }

    #[test]
    fn to_string() {
        let expr_str = "3.3 3 + round neg 4 +";
        let tokens = expr_str.split_whitespace();
        let expr = FloatExpr::<f32>::from_iter(tokens).unwrap();
        assert_eq!(&expr.to_string(), expr_str);
    }

    use std::convert::From;
    use std::str::FromStr;
    use convert_ref::TryFromRef;

    #[derive(Copy, Clone)]
    struct VarIdx(usize);

    #[derive(Debug)]
    enum VarIdxErr<'a, E> {
        InvalidVariableName(&'a str),
        ConvertErr(E),
    }

    impl<'a> TryFromRef<&'a str> for VarIdx {
        type Err = VarIdxErr<'a, <usize as FromStr>::Err>;

        fn try_from_ref(s: &&'a str) -> Result<Self, Self::Err> {
            match s.chars().next() {
                Some('$') => {
                    match FromStr::from_str(&s[1..]) {
                        Ok(n) => Ok(VarIdx(n)),
                        Err(err) => Err(VarIdxErr::ConvertErr(err)),
                    }
                },
                _ => Err(VarIdxErr::InvalidVariableName(s)),
            }
        }
    }

    impl From<VarIdx> for usize {
        fn from(var_idx: VarIdx) -> Self {
            var_idx.0
        }
    }

    #[test]
    fn simple_variable_expression() {
        let expr_str = "3 4 + $0 -";
        let variables = vec![3.0, 500.0];
        let tokens = expr_str.split_whitespace();
        let expr = VariableFloatExpr::<f32, VarIdx>::from_iter(tokens).unwrap();
        assert_eq!(expr.evaluate_with_variables::<usize, _>(variables), Ok(4.0));
    }
}
