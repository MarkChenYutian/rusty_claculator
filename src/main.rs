use std::collections::VecDeque;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

use std::io;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
enum ClacOperator {
    NativePrint,
    NativeQuit,
    NativeAdd,
    NativeSub,
    NativeMul,
    NativeDiv,
    NativeMod,
    NativePow,
    NativeLt,
    NativeDrop,
    NativeSwap,
    NativeRot,
    NativeIf,
    NativePick,
    NativeSkip,
    NativeDef,
    NativeEndDef,
    Number(i32),
    Symbol(String)
}

#[derive(Clone)]
struct ClacStatus {
    queue: VecDeque<ClacOperator>,
    stack: VecDeque<i32>,
    funcs: HashMap<String, VecDeque<ClacOperator>>
}

impl fmt::Debug for ClacStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        f.debug_struct("ClacState")
            .field("S", &self.stack)
            .field("Q", &self.queue)
            .field("Fx", &self.funcs)
            .finish()
    }
}

impl ClacStatus {
    fn new() -> ClacStatus {
        ClacStatus {
            queue: VecDeque::new(),
            stack: VecDeque::new(),
            funcs: HashMap::new()
        }
    }
    
    fn push_queue(mut prev_status: ClacStatus, 
                  new_queue: & VecDeque<ClacOperator>) -> ClacStatus {
        let mut idx : usize = new_queue.len() - 1;
        loop {
            let token: ClacOperator = match new_queue.get(idx) {
                Some(tok) => tok.clone(),
                None => { break; }
            };
            prev_status.queue.push_front(token);
            if idx == 0 { break; }
            idx -= 1;
        }
        prev_status
    }

    fn def_func(mut prev_status: ClacStatus, new_operator: String) -> ClacStatus{
        let mut func_queue: VecDeque<ClacOperator> = VecDeque::new();
        loop {
            match prev_status.queue.pop_front() {
                Some(tok) => {
                    match tok {
                        ClacOperator::NativeEndDef => { break; }
                        _ => { func_queue.push_back(tok); }
                    }
                },
                None => { panic!("Unexpected EOF. Failed to define func {}", new_operator); }
            }
        }
        prev_status.funcs.insert(new_operator, func_queue);
        prev_status
    }

    fn exec(mut prev_status: ClacStatus) -> ClacStatus {
        match prev_status.queue.pop_front() {
            Some(tok) => match tok {
                ClacOperator::NativePrint => {
                    match prev_status.stack.pop_front() {
                        Some(num) => { println!("{}", num); }
                        None => { panic!("Not enough operand for Print"); }
                    }
                },
                ClacOperator::NativeQuit => {
                    println!("\n========== Rusty Clac Exit  ==========\n");
                    panic!("Quit Called manually");
                },
                ClacOperator::NativeAdd => {
                    match (prev_status.stack.pop_front(), prev_status.stack.pop_front()) {
                        (Some(y), Some(x)) => {
                            prev_status.stack.push_front(x + y);
                        },
                        _ => { panic!("Not enough operand for Add"); }
                    }
                },
                ClacOperator::NativeSub => {
                    match (prev_status.stack.pop_front(), prev_status.stack.pop_front()) {
                        (Some(y), Some(x)) => {
                            prev_status.stack.push_front(x - y);
                        },
                        _ => { panic!("Not enough operand for Sub"); }
                    }
                },
                ClacOperator::NativeMul => {
                    match (prev_status.stack.pop_front(), prev_status.stack.pop_front()) {
                        (Some(y), Some(x)) => {
                            prev_status.stack.push_front(x * y);
                        },
                        _ => { panic!("Not enough operand for Mul"); }
                    }
                },
                ClacOperator::NativeDiv => {
                    match (prev_status.stack.pop_front(), prev_status.stack.pop_front()) {
                        (Some(y), Some(x)) => {
                            if (x == i32::MIN && y == -1) || (y == 0) {
                                panic!("Divide by Zero");
                            }
                            prev_status.stack.push_front(x / y);
                        },
                        _ => { panic!("Not enough operand for Div"); }
                    }
                },
                ClacOperator::NativeMod => {
                    match (prev_status.stack.pop_front(), prev_status.stack.pop_front()) {
                        (Some(y), Some(x)) => {
                            if (x == i32::MIN && y == -1) || (y == 0) {
                                panic!("Divide by Zero");
                            }
                            prev_status.stack.push_front(x % y);
                        },
                        _ => { panic!("Not enough operand for Mod"); }
                    }
                },
                ClacOperator::NativePow => {
                    match (prev_status.stack.pop_front(), prev_status.stack.pop_front()) {
                        (Some(y), Some(x)) => {
                            match y.try_into() {
                                Ok(res) => {prev_status.stack.push_front(x.pow(res));}
                                Err(_err) => { panic!("Unable to do calculate negative Pow"); }
                            }
                        },
                        _ => { panic!("Not enough operand for Pow"); }
                    }
                },
                ClacOperator::NativeLt => {
                    match (prev_status.stack.pop_front(), prev_status.stack.pop_front()) {
                        (Some(y), Some(x)) => {
                            if x < y {
                                prev_status.stack.push_front(1);
                            } else {
                                prev_status.stack.push_front(0);
                            }
                        },
                        _ => { panic!("Not enough operand for Lt"); }
                    }
                },
                ClacOperator::NativeDrop => {
                    match prev_status.stack.pop_front() {
                        Some(_x) => {},
                        _ => { panic!("Not enough operand to Drop"); }
                    }
                },
                ClacOperator::NativeSwap =>{
                    match (prev_status.stack.pop_front(), prev_status.stack.pop_front()) {
                        (Some(y), Some(x)) => {
                            prev_status.stack.push_front(y);
                            prev_status.stack.push_front(x);
                        },
                        _ => { panic!("Not enough operand to Swap"); }
                    }
                },
                ClacOperator::NativeRot => {
                    match ( prev_status.stack.pop_front(),
                            prev_status.stack.pop_front(),
                            prev_status.stack.pop_front()) {
                        (Some(z), Some(y), Some(x)) => {
                            prev_status.stack.push_front(y);
                            prev_status.stack.push_front(z);
                            prev_status.stack.push_front(x);
                        },
                        _ => { panic!("Not enough operand to Rot"); }
                    }
                },
                ClacOperator::NativeIf => {
                    match prev_status.stack.pop_front() {
                        Some(val) => {
                            if val == 0 {
                                let mut cnt: i32 = 3;
                                while cnt > 0 {
                                    match prev_status.queue.pop_front() {
                                        Some(_tok) => {},
                                        None => { panic!("Not enough operand for If to skip"); }
                                    }
                                    cnt -= 1;
                                }
                            }
                        },
                        None => { panic!("Not enough operand for If to compare"); }
                    }
                },
                ClacOperator::NativePick => {
                    match prev_status.stack.pop_front() {
                        Some(val) => {
                            let mut cnt: i32 = val;
                            let mut temp_stack: VecDeque<i32> = VecDeque::new();
                            while cnt > 0 {
                                match prev_status.stack.pop_front() {
                                    Some(num) => { temp_stack.push_front(num); },
                                    None => { panic!("Not enough operand to Pick"); }
                                };
                                cnt -= 1;
                            }
                            let picked: i32 = match temp_stack.get(0) {
                                Some(val) => *val,
                                None => { panic!("Not enough operand to Pick"); }
                            };
                            //TODO: Get the nth token on stack, then push back everything
                            while cnt < val {
                                match temp_stack.pop_front() {
                                    Some(num) => {prev_status.stack.push_front(num)},
                                    None => { panic!("Not enough operand to Pick"); }
                                };
                                cnt += 1;
                            }
                            prev_status.stack.push_front(picked);
                        },
                        None => { panic!("Not enough operand for Pick"); }
                    }
                },
                ClacOperator::NativeSkip => {
                    match prev_status.stack.pop_front() {
                        Some(val) => {
                            let mut cnt: i32 = val;
                            while cnt > 0 {
                                match prev_status.queue.pop_front() {
                                    Some(_tok) => {},
                                    None => { panic!("Not enough operand to Skip"); }
                                }
                                cnt -= 1;
                            }
                        },
                        None => { panic!("Not enough operand for Skip"); }
                    }
                },
                ClacOperator::NativeDef => {
                    match prev_status.queue.pop_front() {
                        Some(tok) => {
                            match tok {
                                ClacOperator::Symbol(symbol) => {
                                    return ClacStatus::def_func(prev_status, symbol);
                                },
                                _ => { panic!("Cannot overwrite native instructions"); }
                            }
                        },
                        None => { panic!("Not enough token to define function"); }
                    }
                },
                ClacOperator::NativeEndDef => {
                    panic!("Unexpected NativeEndDef ';' token.");
                }
                ClacOperator::Number(tok) => {
                    prev_status.stack.push_front(tok);
                },
                ClacOperator::Symbol(tok) => {
                    let load_queue = match prev_status.funcs.entry(tok) {
                        Entry::Occupied(entry) => { entry.get().clone() },
                        Entry::Vacant(_) => { panic!("Undefined symbol, failed to parse"); }
                    };
                    return ClacStatus::push_queue(prev_status, &load_queue);
                }
            },
            None => { panic!("Can't exeucte on an empty queue"); }
        };
        prev_status
    }

    fn run( mut prev_status: ClacStatus,
            input_queue: VecDeque<ClacOperator>,
            trace: bool ) -> ClacStatus {
        prev_status = ClacStatus::push_queue(prev_status, &input_queue);
        loop {
            prev_status = ClacStatus::exec(prev_status);
            if trace { println!("|      {:?}", prev_status); }
            if prev_status.queue.len() == 0 { break; }
        }
        prev_status
    }
}

impl ClacOperator {
    fn parse(tok: &str) -> ClacOperator{
        match tok {
            "print" => ClacOperator::NativePrint,
            "quit"  => ClacOperator::NativeQuit,
            "+"     => ClacOperator::NativeAdd,
            "-"     => ClacOperator::NativeSub,
            "*"     => ClacOperator::NativeMul,
            "/"     => ClacOperator::NativeDiv,
            "%"     => ClacOperator::NativeMod,
            "**"    => ClacOperator::NativePow,
            "<"     => ClacOperator::NativeLt,
            "drop"  => ClacOperator::NativeDrop,
            "swap"  => ClacOperator::NativeSwap,
            "rot"   => ClacOperator::NativeRot,
            "if"    => ClacOperator::NativeIf,
            "pick"  => ClacOperator::NativePick,
            "skip"  => ClacOperator::NativeSkip,
            ":"     => ClacOperator::NativeDef,
            ";"     => ClacOperator::NativeEndDef,
            _       =>  {
                    match tok.trim().parse() {
                        Ok(val) => ClacOperator::Number(val),
                        Err(_err) => ClacOperator::Symbol(String::from(tok))
                    }
            }
        }
    }
}

fn get_tokens(s: String) -> VecDeque<ClacOperator> {
    let mut tokens: VecDeque<ClacOperator> = VecDeque::new();
    let mut token_gen = s.split_whitespace();
    loop {
        match token_gen.next() {
            Some(tok) => { tokens.push_back(ClacOperator::parse(tok)); },
            None => { break; }
        }
    }
    tokens
}

fn main() {
    println!("Rusty Clac - v0.1");
    println!("By Yutian Chen");
    println!("Any similarity between Rusty Clac and 15-122 Programming homework - Clac and Exp is *purely coincidental*");

    println!("\n========== Rusty Clac Start ==========\n");

    let mut status = ClacStatus::new();

    loop {
        let mut input: String = String::new();

        println!("\n");

        io::stdin()
            .read_line(&mut input)
            .expect("Can't read the input, re-enter");
        
        let input_queue = get_tokens(input);

        println!("{:?}", input_queue);

        status = ClacStatus::run(status,
                                 input_queue,
                                 false);
    }
}
