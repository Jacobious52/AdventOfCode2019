use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
enum Mode {
    Pos,
    Val,
}

#[derive(Debug, Copy, Clone)]
struct Param {
    mode: Mode,
    val: isize,
}

impl fmt::Display for Param {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.mode {
            Mode::Pos => write!(f, "${}", self.val),
            Mode::Val => write!(f, "{}", self.val),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Instr {
    Add(usize, Param, Param, usize),
    Mul(usize, Param, Param, usize),
    In(usize, usize),
    Out(usize, Param),
    Halt,
}

impl fmt::Display for Instr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instr::Add(_, a, b, dest) => write!(f, "${} = {} + {}", dest, a, b),
            Instr::Mul(_, a, b, dest) => write!(f, "${} = {} * {}", dest, a, b),
            Instr::In(_, dest) => write!(f, "IN => ${}", dest),
            Instr::Out(_, a) => writeln!(f, "{} -> OUT", a),
            Instr::Halt => writeln!(f, "HALT"),
        }
    }
}

#[macro_export]
macro_rules! addr {
    ($i:expr) => {
        $i as usize
    };
}

#[derive(Clone)]
pub struct Interpreter {
    memory: Vec<isize>,
    pc: usize,
}

impl Interpreter {
    pub fn new(memory: Vec<isize>) -> Interpreter {
        Interpreter { memory, pc: 0 }
    }

    pub fn set(&mut self, idx: isize, val: isize) {
        self.memory[addr!(idx)] = val;
    }

    fn param(&self, param: Param) -> isize {
        match param.mode {
            Mode::Pos => self.memory[addr!(param.val)],
            Mode::Val => param.val,
        }
    }

    pub fn eval<I>(mut self, input: I, output: &mut Vec<isize>, debug: bool) -> Interpreter
    where
        I: IntoIterator<Item = isize> + Clone,
    {
        while self.pc < self.memory.len() {
            let intr = self.next_instr();

            if debug {
                println!("{}: {}", self.pc, intr);
            }

            match intr {
                Instr::Add(_, a, b, dest) => self.memory[dest] = self.param(a) + self.param(b),
                Instr::Mul(_, a, b, dest) => self.memory[dest] = self.param(a) * self.param(b),

                Instr::In(_, dest) => {
                    self.memory[dest] = input
                        .clone()
                        .into_iter()
                        .next()
                        .expect("input ran out of values")
                }
                Instr::Out(_, a) => output.push(self.param(a)),

                Instr::Halt => break,
            };
            self.advance(intr);
        }
        self
    }

    pub fn first(&self) -> isize {
        self.memory[0]
    }

    pub fn current(&self) -> isize {
        self.memory[self.pc]
    }

    fn get_op(&self) -> (usize, usize) {
        let opcode = self.memory[self.pc] as usize;
        // get least most 2 digits
        (opcode % 100, opcode / 100)
    }

    fn get_param(&self, op: usize, rel_pos: u32) -> Param {
        let code = (op / usize::pow(10, rel_pos - 1)) % 10;
        let mode = match code {
            0 => Mode::Pos,
            1 => Mode::Val,
            _ => panic!("unknown mode {}", code),
        };

        Param {
            mode,
            val: self.memory[self.pc + rel_pos as usize],
        }
    }

    fn next_instr(&self) -> Instr {
        let (op, opcode) = self.get_op();
        //dbg!(self.memory[self.pc], op, opcode);
        match op {
            1 => Instr::Add(
                4,
                self.get_param(opcode, 1),
                self.get_param(opcode, 2),
                addr!(self.memory[self.pc + 3]),
            ),
            2 => Instr::Mul(
                4,
                self.get_param(opcode, 1),
                self.get_param(opcode, 2),
                addr!(self.memory[self.pc + 3]),
            ),
            3 => Instr::In(2, addr!(self.memory[self.pc + 1])),
            4 => Instr::Out(2, self.get_param(opcode, 1)),
            99 => Instr::Halt,
            _ => panic!("unknown instr: {}", op),
        }
    }

    fn advance(&mut self, intr: Instr) {
        self.pc += match intr {
            Instr::Add(w, _, _, _) => w,
            Instr::Mul(w, _, _, _) => w,
            Instr::In(w, _) => w,
            Instr::Out(w, _) => w,
            Instr::Halt => 1,
        };
    }
}

impl FromStr for Interpreter {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let memory: Result<Vec<isize>, _> = s.split(',').map(|c| c.parse()).collect();

        Ok(Interpreter {
            memory: memory?,
            pc: 0,
        })
    }
}
