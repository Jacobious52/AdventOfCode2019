use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Clone)]
pub struct Interpreter {
    memory: Vec<usize>,
    pc: usize,
}

#[derive(Debug, Copy, Clone)]
enum Instr {
    Add(usize, usize, usize, usize),
    Mul(usize, usize, usize, usize),
    Halt,
}

impl Interpreter {
    pub fn new(memory: Vec<usize>) -> Interpreter {
        Interpreter { memory, pc: 0 }
    }

    pub fn set(&mut self, idx: usize, val: usize) {
        self.memory[idx] = val;
    }

    pub fn eval(mut self) -> Interpreter {
        while self.pc < self.memory.len() {
            let intr = self.next_instr();
            match intr {
                Instr::Add(_, a, b, dest) => self.memory[dest] = self.memory[a] + self.memory[b],
                Instr::Mul(_, a, b, dest) => self.memory[dest] = self.memory[a] * self.memory[b],
                Instr::Halt => {
                    break;
                }
            };
            self.advance(intr);
        }
        self
    }

    pub fn first(&self) -> usize {
        self.memory[0]
    }

    pub fn current(&self) -> usize {
        self.memory[self.pc]
    }

    fn next_instr(&self) -> Instr {
        match self.memory[self.pc] {
            1 => Instr::Add(
                4,
                self.memory[self.pc + 1],
                self.memory[self.pc + 2],
                self.memory[self.pc + 3],
            ),
            2 => Instr::Mul(
                4,
                self.memory[self.pc + 1],
                self.memory[self.pc + 2],
                self.memory[self.pc + 3],
            ),
            99 => Instr::Halt,
            _ => panic!("unknown instr"),
        }
    }

    fn advance(&mut self, intr: Instr) {
        self.pc += match intr {
            Instr::Add(w, _, _, _) => w,
            Instr::Mul(w, _, _, _) => w,
            Instr::Halt => 1,
        };
    }
}

impl FromStr for Interpreter {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let memory: Result<Vec<usize>, _> = s.split(',').map(|c| c.parse()).collect();

        Ok(Interpreter {
            memory: memory?,
            pc: 0,
        })
    }
}
