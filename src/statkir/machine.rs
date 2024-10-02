use std::{collections::HashMap, io::Read};

use super::instructions::Instructions;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MachineState {
    Running,    // normal
    Interupted, // Int is called
    Ended,      // end of program reached
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Machine {
    pc: u64, // program counter, ensured to be 64bits
    state: MachineState,
    calculation_stack: CalculationStack,
    return_stack: ReturnStack,
    runtime_memory: RuntimeMemory,
}

#[allow(dead_code)]
impl Machine {
    fn next(&mut self) {
        self.pc += 1;
    }
    fn skip_im(&mut self) {
        self.pc += size_of::<u64>() as u64;
    }
    fn resume(&mut self) {
        self.state = MachineState::Running;
    }
    pub fn state(&self) -> MachineState {
        self.state.clone()
    }
    fn pop(&mut self) -> u64 {
        self.calculation_stack.pop()
    }
    fn pop_signed(&mut self) -> i64 {
        i64::from_le_bytes(self.calculation_stack.pop().to_le_bytes())
    }
    fn push(&mut self, v: u64) {
        self.calculation_stack.push(v);
    }
    fn push_signed(&mut self, v: i64) {
        self.calculation_stack
            .push(u64::from_le_bytes(v.to_le_bytes()));
    }
    fn r_pop(&mut self) -> u64 {
        self.return_stack.pop()
    }
    fn r_push(&mut self, v: u64) {
        self.return_stack.push(v);
    }
}

#[allow(dead_code)]
pub struct ProgramMemory {
    // program memory
    prog: Vec<u8>,
    // static memory
    // it should be encoded into bytes
    // unchangable
    data: Vec<u8>,
}

#[allow(dead_code)]
impl ProgramMemory {
    pub fn new(prog: Vec<u8>, data: Vec<u8>) -> Self {
        Self { prog, data }
    }

    fn get_opcode_at(&self, index: u64) -> Instructions {
        let opcode = self.prog[index as usize];
        Instructions::of_opcode(opcode)
            .expect(format!("get_opcode error: unsupported opcode {opcode}").as_str())
    }
    fn get_immediate_at(&self, index: u64) -> u64 {
        let mut buf: [u8; 8] = [0; 8];
        let mut bytes = &self.prog[(index as usize)..(index as usize + size_of::<u64>())];
        bytes.read_exact(&mut buf[..size_of::<u64>()]).expect(
            format!("corrupted program memory at {index} read when getting immediate").as_str(),
        );
        u64::from_le_bytes(buf)
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct CalculationStack {
    raw: Vec<u64>,
}

#[allow(dead_code)]
impl CalculationStack {
    fn new() -> CalculationStack {
        CalculationStack { raw: Vec::new() }
    }
    fn push(&mut self, v: u64) {
        self.raw.push(v)
    }
    fn pop(&mut self) -> u64 {
        self.raw
            .pop()
            .expect("pop(?): calculation stack underflowed.")
    }
    fn discard(&mut self) {
        self.raw
            .pop()
            .expect("discard(?): calculation stack underflowed.");
        ()
    }
    fn dup(&mut self) {
        let a = self
            .raw
            .last()
            .expect("dup(?): calculation stack underflowed.");
        self.raw.push(a.clone())
    }
    fn swap(&mut self) {
        let a = self
            .raw
            .pop()
            .expect("swap(?, ?): calculation stack underflowed.");
        let b = self
            .raw
            .pop()
            .expect("swap(a, ?): calculation stack underflowed.");
        self.raw.push(a);
        self.raw.push(b);
    }
    fn over(&mut self) {
        let a = self
            .raw
            .pop()
            .expect("over(?, ?): calculation stack underflowed.");
        let b = self
            .raw
            .pop()
            .expect("over(a, ?): calculation stack underflowed.");
        self.raw.push(b);
        self.raw.push(a);
        self.raw.push(b.clone());
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct ReturnStack {
    raw: Vec<u64>,
}

#[allow(dead_code)]
impl ReturnStack {
    fn new() -> ReturnStack {
        ReturnStack { raw: Vec::new() }
    }
    fn push(&mut self, v: u64) {
        self.raw.push(v)
    }
    fn pop(&mut self) -> u64 {
        self.raw.pop().expect("return stack underflowed.")
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct RuntimeMemory {
    // addr, value
    raw: HashMap<u64, u64>,
}

#[allow(dead_code)]
impl RuntimeMemory {
    fn new() -> RuntimeMemory {
        RuntimeMemory {
            raw: HashMap::new(),
        }
    }
}

#[allow(dead_code)]
impl Machine {
    pub fn new() -> Machine {
        Machine {
            pc: 0u64,
            state: MachineState::Running,
            calculation_stack: CalculationStack::new(),
            return_stack: ReturnStack::new(),
            runtime_memory: RuntimeMemory::new(),
        }
    }

    pub fn run_program(&mut self, program: &ProgramMemory) {
        let instruct = program.get_opcode_at(self.pc);
        match instruct {
            Instructions::Nop => {}
            Instructions::Interupt => self.state = MachineState::Interupted,
            Instructions::FromR => {
                let t = self.r_pop();
                self.push(t);
            }
            Instructions::Swap => self.calculation_stack.swap(),
            Instructions::Over => self.calculation_stack.over(),
            Instructions::Dup => self.calculation_stack.dup(),
            Instructions::Discard => self.calculation_stack.discard(),
            Instructions::Imme => {
                let im = program.get_immediate_at(self.pc + 1);
                self.push(im);
                self.skip_im(); // pc + 8
            }
            Instructions::Store => {
                todo!()
            }
            Instructions::Load => {
                todo!()
            }
            Instructions::J => {
                let addr = program.get_immediate_at(self.pc + 1);
                self.pc = addr;
            }
            Instructions::Jz => {
                let addr = program.get_immediate_at(self.pc + 1);
                let a = self.pop();
                match a {
                    0 => self.pc = addr,
                    _ => (),
                }
            }
            Instructions::Jnz => {
                let addr = program.get_immediate_at(self.pc + 1);
                let a = self.pop();
                match a {
                    0 => (),
                    _ => self.pc = addr,
                }
            }
            Instructions::Ja => {
                let addr = self.pop();
                self.pc = addr;
            }
            Instructions::Add => {
                let a = self.pop_signed();
                let b = self.pop_signed();
                self.push_signed(a + b);
            }
            Instructions::Addu => {
                let a = self.pop();
                let b = self.pop();
                self.push(a + b);
            }
            Instructions::Sub => {
                let a = self.pop_signed();
                let b = self.pop_signed();
                self.push_signed(a - b);
            }
            Instructions::Subu => {
                let a = self.pop();
                let b = self.pop();
                self.push(a - b);
            }
            Instructions::Mul => {
                let a = self.pop_signed();
                let b = self.pop_signed();
                self.push_signed(a * b);
            }
            Instructions::Mulu => {
                let a = self.pop();
                let b = self.pop();
                self.push(a * b);
            }
            Instructions::Div => {
                let a = self.pop_signed();
                let b = self.pop_signed();
                self.push_signed(a / b);
            }
            Instructions::Divu => {
                let a = self.pop();
                let b = self.pop();
                self.push(a / b);
            }
            Instructions::Mod => {
                let a = self.pop_signed();
                let b = self.pop_signed();
                self.push_signed(a % b);
            }
            Instructions::Modu => {
                let a = self.pop();
                let b = self.pop();
                self.push(a % b);
            }
            Instructions::Neg => {
                let t = self.pop_signed();
                self.push_signed(-t);
            }
            Instructions::Shl => {
                let a = self.pop();
                let b = self.pop();
                self.push(a << b);
            }
            Instructions::Shlr => {
                let a = self.pop();
                let b = self.pop();
                self.push(a >> b);
            }
            Instructions::Shar => {
                let a = self.pop_signed();
                let b = self.pop();
                self.push_signed(a >> b);
            }
            Instructions::PopCnt => {
                let t = self.pop();
                self.push(t.count_ones() as u64);
            }
            Instructions::Eq => {
                let a = self.pop();
                let b = self.pop();
                self.push((a == b) as u64);
            }
            Instructions::Neq => {
                let a = self.pop();
                let b = self.pop();
                self.push((a != b) as u64);
            }
            Instructions::Lt => {
                let a = self.pop_signed();
                let b = self.pop_signed();
                self.push((a < b) as u64);
            }
            Instructions::Ltu => {
                let a = self.pop();
                let b = self.pop();
                self.push((a < b) as u64);
            }
            Instructions::Leq => {
                let a = self.pop_signed();
                let b = self.pop_signed();
                self.push((a <= b) as u64);
            }
            Instructions::Lequ => {
                let a = self.pop();
                let b = self.pop();
                self.push((a <= b) as u64);
            }
            Instructions::Gt => {
                let a = self.pop_signed();
                let b = self.pop_signed();
                self.push((a > b) as u64);
            }
            Instructions::Gtu => {
                let a = self.pop();
                let b = self.pop();
                self.push((a > b) as u64);
            }
            Instructions::Geq => {
                let a = self.pop_signed();
                let b = self.pop_signed();
                self.push((a >= b) as u64);
            }
            Instructions::Gequ => {
                let a = self.pop();
                let b = self.pop();
                self.push((a >= b) as u64);
            }
            Instructions::Addf => {
                let a = f64::from_bits(self.pop());
                let b = f64::from_bits(self.pop());
                self.push((a + b).to_bits());
            }
            Instructions::Subf => {
                let a = f64::from_bits(self.pop());
                let b = f64::from_bits(self.pop());
                self.push((a - b).to_bits());
            }
            Instructions::Mulf => {
                let a = f64::from_bits(self.pop());
                let b = f64::from_bits(self.pop());
                self.push((a * b).to_bits());
            }
            Instructions::Divf => {
                let a = f64::from_bits(self.pop());
                let b = f64::from_bits(self.pop());
                self.push((a / b).to_bits());
            }
            Instructions::Modf => {
                let a = f64::from_bits(self.pop());
                let b = f64::from_bits(self.pop());
                self.push((a % b).to_bits());
            }
            Instructions::Negf => {
                let a = f64::from_bits(self.pop());
                self.push((-a).to_bits());
            }
            Instructions::Invf => {
                let a = f64::from_bits(self.pop());
                self.push((1. / a).to_bits());
            }
            Instructions::Sqrf => {
                let a = f64::from_bits(self.pop());
                self.push(a.sqrt().to_bits());
            }
            Instructions::Powf => {
                let a = f64::from_bits(self.pop());
                let b = f64::from_bits(self.pop());
                self.push(a.powf(b).to_bits());
            }
            Instructions::Expf => {
                let a = f64::from_bits(self.pop());
                self.push(a.exp().to_bits());
            }
            Instructions::Logf => {
                let a = f64::from_bits(self.pop());
                self.push((a - 1.).ln_1p().to_bits());
            }
            Instructions::Sinf => {
                let a = f64::from_bits(self.pop());
                self.push(a.sin().to_bits());
            }
            Instructions::Cosf => {
                let a = f64::from_bits(self.pop());
                self.push(a.cos().to_bits());
            }
            Instructions::Tanf => {
                let a = f64::from_bits(self.pop());
                self.push(a.tan().to_bits());
            }
            Instructions::ArcSinf => {
                let a = f64::from_bits(self.pop());
                self.push(a.asin().to_bits());
            }
            Instructions::ArcCosf => {
                let a = f64::from_bits(self.pop());
                self.push(a.acos().to_bits());
            }
            Instructions::ArcTanf => {
                let a = f64::from_bits(self.pop());
                self.push(a.atan().to_bits());
            }
            Instructions::Eqf => {
                let a = f64::from_bits(self.pop());
                let b = f64::from_bits(self.pop());
                self.push((a == b) as u64);
            }
            Instructions::Neqf => {
                let a = f64::from_bits(self.pop());
                let b = f64::from_bits(self.pop());
                self.push((a != b) as u64);
            }
            Instructions::Ltf => {
                let a = f64::from_bits(self.pop());
                let b = f64::from_bits(self.pop());
                self.push((a < b) as u64);
            }
            Instructions::Leqf => {
                let a = f64::from_bits(self.pop());
                let b = f64::from_bits(self.pop());
                self.push((a <= b) as u64);
            }
            Instructions::Gtf => {
                let a = f64::from_bits(self.pop());
                let b = f64::from_bits(self.pop());
                self.push((a > b) as u64);
            }
            Instructions::Geqf => {
                let a = f64::from_bits(self.pop());
                let b = f64::from_bits(self.pop());
                self.push((a >= b) as u64);
            }
            Instructions::ItoF => {
                let a = self.pop() as i64;
                self.push((a as f64).to_bits());
            }
            Instructions::FtoI => {
                let a = f64::from_bits(self.pop());
                self.push(u64::from_le_bytes((a as i64).to_le_bytes()));
            }
        };
        self.next();
        if self.pc >= program.prog.len() as u64 {
            self.state = MachineState::Ended
        }
    }
}
