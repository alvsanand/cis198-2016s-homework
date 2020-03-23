use std::io::{self, Write};

use rpn::{self, Stack};

/// Start a read-eval-print loop, which runs until an error or `quit`.
#[allow(unused_must_use)]
pub fn read_eval_print_loop() -> rpn::Result<()> {
    // Create a stack to work on.
    let mut stack = Stack::new();

    loop {
        // Print a user input prompt.
        print!("> ");
        let stdout = io::stdout();
        let mut handle_out = stdout.lock();

        handle_out.flush().map_err(rpn::Error::IO);

        let mut buf = String::new();

        match io::stdin()
            .read_line(&mut buf)
            .map_err(rpn::Error::IO)
            .and(evaluate_line(&mut stack, &buf))
        {
            Err(err) => return rpn::Result::Err(err),
            _ => (),
        };
    }
}

fn parse_operation(val: &str) -> Result<rpn::Op, rpn::Error> {
    match val {
        "+" => Result::Ok(rpn::Op::Add),
        "~" => Result::Ok(rpn::Op::Neg),
        "<->" => Result::Ok(rpn::Op::Swap),
        "=" => Result::Ok(rpn::Op::Eq),
        "#" => Result::Ok(rpn::Op::Rand),
        "quit" => Result::Ok(rpn::Op::Quit),
        _ => Result::Err(rpn::Error::Syntax),
    }
}

fn parse_val(val: &str) -> Result<rpn::Elt, rpn::Error> {
    val.parse::<i32>()
        .map(|v| rpn::Elt::Int(v))
        .or(val.parse::<bool>().map(|v| rpn::Elt::Bool(v)))
        .or(Err(rpn::Error::Syntax))
}

#[allow(unused_must_use)]
fn evaluate_line(stack: &mut Stack, buf: &String) -> rpn::Result<()> {
    // Create an iterator over the tokens.
    let mut tokens = buf.trim().split_whitespace();

    match tokens.next() {
        Some(token) => {
            if let Result::Ok(op) = parse_operation(token) {
                let is_swap = op == rpn::Op::Swap;
                let result = stack.eval(op);

                match result {
                    rpn::Result::Ok(_) if is_swap => {
                        let (y, x) = (stack.pop().unwrap(), stack.pop().unwrap());
                        println!("= {:?} <-> {:?}", x, y);
                    }
                    rpn::Result::Ok(_) => println!("= {:?}", stack.pop().unwrap()),
                    _ => (),
                };
                result
            } else if let Result::Ok(val) = parse_val(token) {
                stack.push(val)
            } else {
                rpn::Result::Err(rpn::Error::Syntax)
            }
        }
        None => rpn::Result::Err(rpn::Error::Syntax),
    }
}

#[cfg(test)]
mod tests {
    use parser::evaluate_line;
    use rpn::{Elt, Error, Stack};

    #[test]
    fn test_evaluate_line_bool() {
        let mut stack = Stack::new();
        let s = "true".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        assert_eq!(stack.pop().unwrap(), Elt::Bool(true));
        let s = "false".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        assert_eq!(stack.pop().unwrap(), Elt::Bool(false));
    }

    #[test]
    fn test_evaluate_line_int() {
        let mut stack = Stack::new();
        let s = "12".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        assert_eq!(stack.pop().unwrap(), Elt::Int(12));
    }

    #[test]
    fn test_evaluate_line_plus() {
        let mut stack = Stack::new();
        let s = "12".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        let s = "13".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        let s = "+".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        assert_eq!(stack.pop().unwrap(), Elt::Int(25));
    }

    #[test]
    fn test_evaluate_line_neg() {
        let mut stack = Stack::new();
        let s = "false".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        let s = "~".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        assert_eq!(stack.pop().unwrap(), Elt::Bool(true));
    }

    #[test]
    fn test_evaluate_line_swap() {
        let mut stack = Stack::new();
        let s = "false".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        let s = "15".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        let s = "<->".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        assert_eq!(stack.pop().unwrap(), Elt::Bool(false));
        assert_eq!(stack.pop().unwrap(), Elt::Int(15));
    }

    #[test]
    fn test_evaluate_line_eq() {
        let mut stack = Stack::new();
        let s = "12".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        let s = "15".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        let s = "=".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        assert_eq!(stack.pop().unwrap(), Elt::Bool(false));
    }

    #[test]
    fn test_evaluate_line_rand() {
        let mut stack = Stack::new();
        let s = "12".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        let s = "#".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        let res = stack.pop();
        assert!(res.is_ok());
        let res = res.unwrap();
        assert!(res >= Elt::Int(0));
        assert!(res < Elt::Int(12));
    }

    #[test]
    fn test_evaluate_line_quit() {
        let mut stack = Stack::new();
        let s = "quit".to_string();
        let res = evaluate_line(&mut stack, &s);
        assert!(res.is_err());
        if let Err(Error::Quit) = res {
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_evaluate_line_bad_parse() {
        let mut stack = Stack::new();
        let s = "~false".to_string();
        let res = evaluate_line(&mut stack, &s);
        assert!(res.is_err());
        if let Err(Error::Syntax) = res {
        } else {
            assert!(false);
        }
    }
}
