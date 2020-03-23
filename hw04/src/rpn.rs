use std::io;
use std::result;
use rand;

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
/// An element of the stack. May be either integer or boolean.
pub enum Elt {
    Int(i32),
    Bool(bool),
}

#[derive(Debug)]
/// An RPN calculator error.
pub enum Error {
    /// Tried to pop from an empty stack.
    Underflow,
    /// Tried to operate on invalid types (e.g. 4 + true)
    Type,
    /// Unable to parse the input.
    Syntax,
    /// Some IO error occurred.
    IO(io::Error),
    /// The user quit the program (with `quit`).
    Quit,
}

#[derive(Debug, PartialEq)]
/// Types of RPN calculator operations.
pub enum Op {
    /// Adds two numbers: pop x, pop y, push x + y.
    Add,
    /// Checks equality of two values: pop x, pop y, push x == y.
    Eq,
    /// Negates a value: pop x, push ~x.
    Neg,
    /// Swaps two values: pop x, pop y, push x, push y.
    Swap,
    /// Computes a random number: pop x, push random number in [0, x).
    Rand,
    /// Quit the calculator.
    Quit,
}

// TODO: Stack.
pub struct Stack(Vec<Elt>);

// TODO: Result.
pub type Result<Elt> = result::Result<Elt, Error>;

impl Stack {
    /// Creates a new Stack
    pub fn new() -> Stack {
        Stack(Vec::new())
    }

    /// Pushes a value onto the stack.
    pub fn push(&mut self, val: Elt) -> Result<()> {
        self.0.push(val);
        Result::Ok(())
    }

    /// Tries to pop a value off of the stack.
    pub fn pop(&mut self) -> Result<Elt> {
        self.0
            .pop()
            .map_or(Result::Err(Error::Underflow), |val| Result::Ok(val))
    }

    fn add(&self, x: Elt, y: Elt) -> Result<Elt> {
        if let Elt::Int(number_x) = x {
            if let Elt::Int(number_y) = y {
                Result::Ok(Elt::Int(number_x + number_y))
            } else {
                Result::Err(Error::Type)
            }
        } else {
            Result::Err(Error::Type)
        }
    }

    fn eq(&self, x: Elt, y: Elt) -> Result<Elt> {
        if let Elt::Int(number_x) = x {
            if let Elt::Int(number_y) = y {
                Result::Ok(Elt::Bool(number_x == number_y))
            } else {
                Result::Err(Error::Type)
            }
        } else {
            Result::Err(Error::Type)
        }
    }

    fn neg(&self, x: Elt) -> Result<Elt> {
        match x {
            Elt::Int(number_x) => Result::Ok(Elt::Int(-number_x)),
            Elt::Bool(bool_x) => Result::Ok(Elt::Bool(!bool_x)),
        }
    }

    fn rand(&mut self, x: Elt) -> Result<Elt> {
        if let Elt::Int(number_x) = x {
            let random_number = (rand::random::<f64>() * number_x as f64) as i32;
            Result::Ok(Elt::Int(random_number))
        } else {
            Result::Err(Error::Type)
        }     
    }

    fn swap(&mut self, x: Elt, y: Elt) -> Result<Elt> {
        self.push(x).and_then(|_| self.push(y)).and_then(|_| Result::Ok(Elt::Bool(true)))        
    }

    /// Tries to evaluate an operator using values on the stack.
    pub fn eval(&mut self, op: Op) -> Result<()> {
        let result = match op {
            Op::Add => {
                let x = self.pop();
                let y = self.pop();

                x.and_then(|x| y.and_then(|y| self.add(x, y)))
            }
            Op::Eq => {
                let x = self.pop();
                let y = self.pop();

                x.and_then(|x| y.and_then(|y| self.eq(x, y)))
            }
            Op::Neg => {
                let x = self.pop();

                x.and_then(|x| self.neg(x))
            }
            Op::Quit => Result::Err(Error::Quit),
            Op::Rand => {
                let x = self.pop();

                x.and_then(|x| self.rand(x))
            }
            Op::Swap => {
                let x = self.pop();
                let y = self.pop();

                x.and_then(|x| y.and_then(|y| self.swap(x, y)))
            }
        };

        match (result, op) {
            (Result::Ok(_), Op::Swap)  => Result::Ok(()),
            (Result::Ok(val), _) => self.push(val),
            (Result::Err(err), _) => Result::Err(err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pop_empty1() {
        let mut s = Stack::new();

        let res = s.pop();
        assert!(res.is_err());
        if let Err(Error::Underflow) = res {
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_pop_empty2() {
        let mut s = Stack::new();
        s.push(Elt::Int(0)).unwrap();

        let res = s.pop();
        assert!(res.is_ok());

        let res = s.pop();
        assert!(res.is_err());
        if let Err(Error::Underflow) = res {
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_eval_add1() {
        let mut s = Stack::new();
        s.push(Elt::Int(1)).unwrap();
        s.push(Elt::Int(1)).unwrap();

        assert!(s.eval(Op::Add).is_ok());
        assert_eq!(s.pop().unwrap(), Elt::Int(2));
    }

    #[test]
    fn test_eval_add2() {
        let mut s = Stack::new();
        s.push(Elt::Int(1)).unwrap();
        s.push(Elt::Bool(false)).unwrap();

        let res = s.eval(Op::Add);
        assert!(res.is_err());
        if let Err(Error::Type) = res {
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_eval_add3() {
        let mut s = Stack::new();
        s.push(Elt::Bool(true)).unwrap();
        s.push(Elt::Bool(false)).unwrap();

        let res = s.eval(Op::Add);
        assert!(res.is_err());
        if let Err(Error::Type) = res {
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_eval_eq1() {
        let mut s = Stack::new();
        s.push(Elt::Int(1)).unwrap();
        s.push(Elt::Int(1)).unwrap();

        assert!(s.eval(Op::Eq).is_ok());
        assert_eq!(s.pop().unwrap(), Elt::Bool(true));
    }

    #[test]
    fn test_eval_eq2() {
        let mut s = Stack::new();
        s.push(Elt::Int(1)).unwrap();
        s.push(Elt::Bool(false)).unwrap();

        let res = s.eval(Op::Eq);
        assert!(res.is_err());
        if let Err(Error::Type) = res {
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_eval_neg1() {
        let mut s = Stack::new();
        s.push(Elt::Int(1)).unwrap();
        assert!(s.eval(Op::Neg).is_ok());
        assert_eq!(s.pop().unwrap(), Elt::Int(-1));
    }

    #[test]
    fn test_eval_neg2() {
        let mut s = Stack::new();
        s.push(Elt::Bool(false)).unwrap();
        assert!(s.eval(Op::Neg).is_ok());
        assert_eq!(s.pop().unwrap(), Elt::Bool(true));
    }

    #[test]
    fn test_eval_swap1() {
        let mut s = Stack::new();
        s.push(Elt::Int(1)).unwrap();
        s.push(Elt::Bool(false)).unwrap();

        assert!(s.eval(Op::Swap).is_ok());
        assert_eq!(s.pop().unwrap(), Elt::Int(1));
        assert_eq!(s.pop().unwrap(), Elt::Bool(false));

        let res = s.pop();
        assert!(res.is_err());
        if let Err(Error::Underflow) = res {
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_eval_swap2() {
        let mut s = Stack::new();
        s.push(Elt::Bool(false)).unwrap();

        let res = s.eval(Op::Swap);
        assert!(res.is_err());
        if let Err(Error::Underflow) = res {
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_eval_rand1() {
        let mut s = Stack::new();
        let i = 20;
        s.push(Elt::Int(i)).unwrap();

        assert!(s.eval(Op::Rand).is_ok());

        let rand_val = s.pop().unwrap();
        assert!(rand_val >= Elt::Int(0));
        assert!(rand_val < Elt::Int(i));
    }

    #[test]
    fn test_eval_rand2() {
        let mut s = Stack::new();
        s.push(Elt::Bool(false)).unwrap();

        let res = s.eval(Op::Rand);
        assert!(res.is_err());
        if let Err(Error::Type) = res {
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_eval_quit() {
        let mut s = Stack::new();

        let res = s.eval(Op::Quit);
        assert!(res.is_err());
        if let Err(Error::Quit) = res {
        } else {
            assert!(false);
        }
    }
}
