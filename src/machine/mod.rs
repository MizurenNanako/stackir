pub mod calculation_stack;
mod machine_actions;
pub mod program_memory;
pub mod return_stack;
pub mod runtime_memory;

use crate::instruction::Instructions;
use crate::machine::calculation_stack::CalculationStack;
use crate::machine::return_stack::ReturnStack;
use crate::machine::runtime_memory::RuntimeMemory;

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
    fn skip_im(&mut self, n: usize) {
        self.pc += n as u64;
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
}
