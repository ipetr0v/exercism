use std::collections::HashMap;

pub type Value = i32;
pub type ForthResult = Result<(), Error>;

#[derive(Debug, Clone)]
enum Word {
    Nop,
    Add, Sub, Mul, Div,
    Dup, Drop, Swap, Over,
    Val(Value),
    Ident(String),
    Err(Error)
}
type Program = Vec<Word>;

#[derive(Debug, PartialEq, Clone)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

struct Definition {
    is_active: bool,
    program: Vec<String>
}

impl Definition {
    fn new() -> Self {
        Self {is_active: false, program: vec![]}
    }

    fn add(&mut self, input: String) {
        self.program.push(input)
    }

    fn pop(&mut self) -> Result<(String, Vec<String>), Error> {
        match self.program.iter().nth(0) {
            Some(x) if self.program.len() > 1 => {
                (x.clone(), self.program[1..])
            },
            _ => Err(Error::InvalidWord)
        }
    }
}

pub struct Forth {
    stack: Vec<Value>,
    defs: HashMap<String, Program>,
    tmp_def: Definition
}

impl Forth {
    pub fn new() -> Forth {
        Forth {
            stack: vec![],
            tmp_def: Definition::new(),
            defs: vec![
                ("+", Word::Add)
                ("-", Word::Sub),
                ("*", Word::Mul),
                ("/", Word::Div),
                ("DUP", Word::Dub),
                ("DROP", Word::Drop),
                ("SWAP", Word::Swap),
                ("OVER", Word::Over)
            ].iter()
             .cloned()
             .map(|(s, op)| (s.to_string(), vec![op]))
             .collect::<HashMap<String, Program>>()
        }
    }

    pub fn stack(&self) -> Vec<Value> {
        self.stack.clone()
    }

    pub fn eval(&mut self, input: &str) -> ForthResult {
        input.split_whitespace()
             .map(|s| self.process(s))
             .flat_map(|w| self.expand(w))
             .try_for_each(|w| self.execute(&w))?;
        if self.tmp_def.is_active {
            Err(Error::InvalidWord)
        } else {
            Ok(())
        }
    }

    fn parse(input: &str) -> Word {
        match input.parse() {
            Ok(value) => Word::Val(value),
            Err(_) => Word::Ident(input)
        }
    }
       
    fn process(&self, raw_input: &str) -> Word {
        let input = raw_input.to_ascii_uppercase();
        if self.tmp_def.is_active {
            match input.as_ref() {
                ":" => Word::Err(Error::InvalidWord)?,
                ";" => match self.define() {
                    Ok() => Word::Nop(()),
                    Err(error) => Word::Err(error)
                },
                _ => {
                    let program = Forth::parse(input)
                    self.tmp_def.pub(program)
                }//Word::Nop(self.tmp_def.push(input))
            }
        } else {
            match input.as_ref() {
                ":" => Word::Nop(self.tmp_def.is_active = true),
                ";" => Word::Err(Error::InvalidWord)?,
                _ => Forth::parse(input)
            }
        }
    }

    fn expand(&self, word: &Word) -> Program {
        match word {
            Word::Ident(ident) =>  match self.get_def(ident) {
                Ok(program) => program,
                Err(error) => vec![error]
            },
            _ => vec![word]
        }
    }

    fn execute(&mut self, word: &Word) -> ForthResult {
        Ok(match word {
            Word::Nop => (),
            Word::Err(error) => Err(error)?,
            Word::Add => {
                let val = self.pop()? + self.pop()?;
                self.push(val);
            },
            Word::Sub => {
                let (val1, val2) = (self.pop()?, self.pop()?);
                self.push(val2 - val1);
            },
            Word::Mul => {
                let val = self.pop()? * self.pop()?;
                self.push(val);
            },
            Word::Div => {
                let (val1, val2) = (self.pop()?, self.pop()?);
                self.push(Forth::div(val2, val1)?)
            },
            Word::Dup => {
                let val = self.pop()?;
                self.push(val);
                self.push(val);
            },
            Word::Drop => {self.pop()?;},
            Word::Swap => {
                let (val1, val2) = (self.pop()?, self.pop()?);
                self.push(val1);
                self.push(val2);
            },
            Word::Over => {
                let (val1, val2) = (self.pop()?, self.pop()?);
                self.push(val2);
                self.push(val1);
                self.push(val2);
            }
        })
    }

    fn push(&mut self, val: Value) {
        self.stack.push(val);
    }

    fn pop(&mut self) -> Result<Value, Error> {
        self.stack.pop().ok_or(Error::StackUnderflow)
    }

    fn div(first: Value, second: Value) -> Result<Value, Error> {
        match second {
            0 => Err(Error::DivisionByZero),
            _ => Ok(first / second)
        }
    }

    fn define(&mut self) -> ForthResult {
        let (name, program) = self.tmp_def.pop()?;
        self.defs.insert(
            name,
            program.iter()
                   .map(|s| Forth::parse(s))
                   .flat_map(|w| self.expand(w))
                   .collect::<Program>()
        );
        self.tmp_def = Definition::new();
        Ok(())
    }

    fn get_def(&self, word: &str) -> Result<Program, Error> {
        self.defs.get(word)
                 .cloned()
                 .ok_or(Error::UnknownWord)
    }
}
