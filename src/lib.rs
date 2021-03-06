use std::fs::File;
use std::io::Read;

pub fn load_module(path: &'static str) -> Vec<u8> {
    let mut file = File::open(path).unwrap();
    let mut program: Vec<u8> = Vec::new();
    
    file.read_to_end(&mut program).unwrap();

    program
}

#[derive(Debug, Copy, Clone)]
pub enum Error {
    StackUnderflow,
    TypeMismatch,
    InvalidInstruction,
}

pub trait AtomExtender {
    fn atom(&mut self, u8, &mut Stack) -> Result<(),Error>;
}

pub struct NullExtender {}
impl AtomExtender for NullExtender {
    fn atom(&mut self, _: u8, _: &mut Stack) -> Result<(),Error> {
        Err(Error::InvalidInstruction)
    }
}


impl Error {
    pub fn to_string(&self) -> &'static str {
        match self {
            &Error::StackUnderflow => {return "Stack Underflow";},
            &Error::TypeMismatch   => {return "Type Mismatch";},
            &Error::InvalidInstruction => {return "Invalid Instruction";},
        }
    }
}



#[derive(Debug, Copy, Clone)]
///Represents a piece of Forth data, either an int or a float.
pub enum Data {
    Int(i64),
    Float(f64),
}

#[derive(Debug, Copy, Clone)]
///Represents a homogeneous pair of Data.
pub enum Pair {
    Int(i64,i64),
    Float(f64,f64),
}

///The Forth stack.
pub struct Stack {
    stack: Vec<Data>,
}

impl Stack {
    ///Initialize an empty stack.
    pub fn new() -> Stack {
        Stack {
            stack: Vec::new()
        }
    }


    ///Get the length of the stack.
    pub fn len(&self) -> usize {self.stack.len()}


    ///Push an item to the stack.
    pub fn push(&mut self, value: Data) {
        self.stack.push(value);
    }

    ///Pop an item from the stack.
    pub fn pop(&mut self) -> Result<Data,Error> {
        match self.stack.pop() {
            Some(n) => Ok(n),
            None    => Err(Error::StackUnderflow),
        }
    }

    ///Pop two items of the same type.
    pub fn pop_two(&mut self) -> Result<Pair,Error> {
        let a = self.stack.pop();
        let b = self.stack.pop();

        if let None = a {return Err(Error::StackUnderflow);}
        if let None = b {return Err(Error::StackUnderflow);}

        let a = a.unwrap();
        let b = b.unwrap();

        match (a,b) {
            (Data::Float(x),Data::Float(y)) => Ok(Pair::Float(x,y)),
            (Data::Int(x),Data::Int(y)) => Ok(Pair::Int(x,y)),
            (Data::Float(_),Data::Int(_)) => Err(Error::TypeMismatch),
            (Data::Int(_),Data::Float(_)) => Err(Error::TypeMismatch),
        }
    }

    ///Cast TOS to int. Int to int is valid.
    pub fn cast_to_int(&mut self) -> Result<(),Error> {
        let value = self.pop();

        let value = match value {
            Err(n) => { return Err(n);},
            Ok(n)  => { n }
        };

        match value {
            Data::Int(_) => {self.push(value);},
            Data::Float(n) => {self.push(Data::Int(n as i64));}
        }

        return Ok(());
    }

    ///Cast TOS to float. Float to float is valid.
    pub fn cast_to_float(&mut self) -> Result<(),Error> {
        let value = self.pop();

        let value = match value {
            Err(n) => { return Err(n);},
            Ok(n)  => { n }
        };

        match value {
            Data::Int(n) => {self.push(Data::Float(n as f64));},
            Data::Float(_) => {self.push(value);}
        }

        return Ok(());
    }

    ///Duplicate TOS.
    pub fn dup(&mut self) -> Result<(),Error> {
        let value = self.pop();

        let value = match value {
            Err(n) => { return Err(n);},
            Ok(n)  => { n }
        };

        self.push(value);
        self.push(value);

        Ok(())
    }

    ///Swap TOS with NOS.
    pub fn swap(&mut self) -> Result<(), Error> {
        let x = self.pop();
        let y = self.pop();

        let x = match x {
            Err(n) => { return Err(n);},
            Ok(n)  => { n }
        };

        let y = match y {
            Err(n) => { return Err(n);},
            Ok(n)  => { n }
        };

        self.push(x);
        self.push(y);

        Ok(())
    }

    ///Duplicate NOS, placing duplicate value above TOS.
    pub fn over(&mut self) -> Result<(), Error> {
        let x = self.pop();
        let y = self.pop();

        let x = match x {
            Err(n) => { return Err(n);},
            Ok(n)  => { n }
        };

        let y = match y {
            Err(n) => { return Err(n);},
            Ok(n)  => { n }
        };

        self.push(y);
        self.push(x);
        self.push(y);

        Ok(())
    }

    pub fn add(&mut self) -> Result<(),Error> {
        let values = self.pop_two(); 

        let values = match values {
            Err(n) => { return Err(n);},
            Ok(n)  => { n }
        };

        match values {
            Pair::Int(x,y) => {self.push(Data::Int(x+y));}
            Pair::Float(x,y) => { self.push(Data::Float(x+y));}
        }

        Ok(())
    }

    pub fn sub(&mut self) -> Result<(),Error> {
        let values = self.pop_two(); 

        let values = match values {
            Err(n) => { return Err(n);},
            Ok(n)  => { n }
        };

        match values {
            Pair::Int(x,y) => {self.push(Data::Int(y-x));}
            Pair::Float(x,y) => { self.push(Data::Float(y-x));}
        }

        Ok(())
    }

    pub fn mul(&mut self) -> Result<(),Error> {
        let values = self.pop_two(); 

        let values = match values {
            Err(n) => { return Err(n);},
            Ok(n)  => { n }
        };

        match values {
            Pair::Int(x,y) => {self.push(Data::Int(y*x));}
            Pair::Float(x,y) => { self.push(Data::Float(y*x));}
        }

        Ok(())
    }

    pub fn div(&mut self) -> Result<(),Error> {
        let values = self.pop_two(); 

        let values = match values {
            Err(n) => { return Err(n);},
            Ok(n)  => { n }
        };

        match values {
            Pair::Int(x,y) => {self.push(Data::Int(y/x));}
            Pair::Float(x,y) => { self.push(Data::Float(y/x));}
        }

        Ok(())
    }

    pub fn modulus(&mut self) -> Result<(),Error> {
        let values = self.pop_two(); 

        let values = match values {
            Err(n) => { return Err(n);},
            Ok(n)  => { n }
        };

        match values {
            Pair::Int(x,y) => {self.push(Data::Int(y%x));}
            Pair::Float(x,y) => { self.push(Data::Float(y%x));}
        }

        Ok(())
    }

}

/// Run some code. The stack and memory parameters can be non-empty;
/// this is how data is passed to a Forth function. Since they are
/// only borrowed return values can be extracted from them.
///
/// PC should be set to the beginning of one of the words in memory.
/// A return without a branch triggers a full return, no longer a Return Stack Underflow.

pub fn run<T: AtomExtender>(
            code: &Vec<u8>,
            stack: &mut Stack,
            mut pc: usize,
            mut extender: T,
            memory: &mut Vec<Data>
            ) -> Result<(),(usize,Error)> {

    let mut rstack: Vec<usize> = Vec::new();

    let mut value: i64 = 0;
    let mut divider: f64 = 1.0;

    while pc < code.len() {
        let instruction = code[pc];
        pc += 1;

        match instruction {
            10 => {},
            13 => {},   //Carriage Returns and Line feeds are ignored
            32 => {},   //Tabs are not allowed but spaces are.
            34 => {     //Double quote. Push constant as float
                let v = value as f64;
                stack.push(Data::Float(v / divider));
            },
            35 => {     //Pound sign. Load constant.
                value = 0;
                divider = 1.0;
            },
            36 => {     //Dollar sign. Invert constant.
                value = -value;
            },
            37 => {     //Percent sign. Modulus.
                if let Err(n) = stack.modulus() { return Err((pc, n)); }
            }
            39 => {     //Single quote. Push constant as integer.
                stack.push(Data::Int(value));
            },
            42 => {     //Asterisk. Multiply.
                if let Err(n) = stack.mul() { return Err((pc, n)); }
            },
            43 => {     //Plus sign. Add.
                if let Err(n) = stack.add() { return Err((pc, n)); }
            },
            45 => {     //Minus sign. Subtract.
                if let Err(n) = stack.sub() { return Err((pc, n)); }
            },
            46 => {     //Period. Increase the divider by three orders of magnitude.
                divider *= 1000.0;
            },
            47 => {     //Slash. Divide.
                if let Err(n) = stack.div() { return Err((pc, n)); }
            },
            48...57 => { //Numeral.
                value *= 10;
                value += (instruction as i64) - 48;
            },
            59 => {     //Semicolon. Return
                let home = match rstack.pop() {
                    Some(n) => n,
                    None    => { return Ok(()) }
                };

                pc = home; 
            },
            82 => {     //"R" Read from memory
                let value = stack.pop();
                
                let value = match value {
                    Err(n) => { return Err((pc,n));},
                    Ok(n)  => { n }
                };

                match value {
                    Data::Float(_) => { return Err((pc, Error::TypeMismatch)); }
                    Data::Int(n) => {
                        let val = memory[(n as usize) % memory.len()];
                        stack.push(val);
                    }
                }
            },
            87 => {
                let address = stack.pop();
                let value = stack.pop();

                let address = match address {
                    Err(n) => { return Err((pc,n));},
                    Ok(n)  => { n }
                };

                let value = match value {
                    Err(n) => { return Err((pc,n));},
                    Ok(n)  => { n }
                };

                match address {
                    Data::Float(_) => { return Err((pc, Error::TypeMismatch)); }
                    Data::Int(n) => {
                        let addr = (n as usize) % memory.len();
                        memory[addr] = value;
                    }
                }
            },
            98  => {    //"b". Jump to address.
                //println!("{}",stack.len());
                let value = stack.pop();
                
                let value = match value {
                    Err(n) => { return Err((pc,n));},
                    Ok(n)  => { n }
                };

                match value {
                    Data::Float(_) => { return Err((pc, Error::TypeMismatch)); }
                    Data::Int(n) => {
                        pc = n as usize;
                    }
                }
            },
            99 => {     //"c". Call address
                //println!("{}",stack.len());
                let value = stack.pop();
                
                let value = match value {
                    Err(n) => { return Err((pc,n));},
                    Ok(n)  => { n }
                };

                match value {
                    Data::Float(_) => { return Err((pc, Error::TypeMismatch)); }
                    Data::Int(n) => {
                        rstack.push(pc);
                        pc = n as usize;
                    }
                }
            },
            100 => {    //"d". Duplicate.
                if let Err(n) = stack.dup() { return Err((pc, n)); }

            },
            112 => {    //"p". Debug. Print type of variable, and value.

                let value = stack.pop();
                
                let value = match value {
                    Err(n) => { return Err((pc,n));},
                    Ok(n)  => { n }
                };

                match value {
                    Data::Int(n) => println!("Int:{}",n),
                    Data::Float(n) => println!("Float:{}",n)
                }
            },
            114 => {    //"r" Drop.
                if let Err(n) = stack.pop() { return Err((pc, n)); }

            },

            115 => {    //"s" Swap.
                if let Err(n) = stack.swap() { return Err((pc, n)); }

            }
            118 => {    //"v" Over.
                if let Err(n) = stack.over() { return Err((pc, n)); }
            }
            121 => {    //"y" Jump if non-zero.
                let address = stack.pop();

                let address = match address { Err(n) => {return Err((pc,n));}, Ok(n) => {n} };

                let data = stack.pop();

                let data = match data { Err(n) => {return Err((pc,n));}, Ok(n) => {n} };

                let address = match address { Data::Float(_) => {return Err((pc, Error::TypeMismatch));}, Data::Int(n) => n as usize };

                let condition = match data {
                    Data::Float(n) => if n != 0.0 {true} else {false},
                    Data::Int(n)   => if n != 0 {true} else {false}
                };

                if condition { pc = address; }
            },
            122 => {    //"z" Jump if zero.
                let address = stack.pop();

                let address = match address { Err(n) => {return Err((pc,n));}, Ok(n) => {n} };

                let data = stack.pop();

                let data = match data { Err(n) => {return Err((pc,n));}, Ok(n) => {n} };

                let address = match address { Data::Float(_) => {return Err((pc, Error::TypeMismatch));}, Data::Int(n) => n as usize };

                let condition = match data {
                    Data::Float(n) => if n == 0.0 {true} else {false},
                    Data::Int(n)   => if n == 0 {true} else {false}
                };

                if condition { pc = address; }
            },
            _ => {
                if let Err(n) = extender.atom(instruction, stack) { return Err((pc, n)); }
            },

        }
    }

    Ok(())

}

#[cfg(test)]
mod tests {
    use Stack;
    use Error;
    use Data;

    #[test]
    fn it_works() {
        let mut s = Stack::new();

        let v = s.pop();

        assert!(if let Err(Error::StackUnderflow) = v {true} else {false});

        s.push(Data::Int(5));

        let five = s.pop();

        match five {
            Err(_) => panic!("Error"),
            Ok(n) => match n {
                Data::Int(v) => match v {
                    5 => {},
                    _ => panic!("No good"),
                },
                Data::Float(_) => panic!("Wrong type")
            }
        }

        s.push(Data::Int(5));
        s.push(Data::Float(2.0));

        let pair = s.pop_two();

        assert!(if let Err(Error::TypeMismatch) = pair {true} else {false});

        assert!(if let Err(Error::StackUnderflow) = s.cast_to_int() {true} else {false});

        s.push(Data::Int(2));
        s.push(Data::Int(6));

        assert!(if let Ok(()) = s.add() {true} else {false});

        let eight = s.pop();

        match eight {
            Err(_) => panic!("Error"),
            Ok(n) => match n {
                Data::Int(v) => match v {
                    8 => {},
                    _ => panic!("No good"),
                },
                Data::Float(_) => panic!("Wrong type")
            }
        }

    }
}
