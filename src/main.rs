use stackir::core as stk;
use stk::{
    instructions::Instructions as I,
    machine::{Machine, MachineState, ProgramMemory},
};

fn main() {
    use stk::instructions::ProgramMaker;
    let mut ins = Vec::new();
    ins
        // push 12
        .add_ins(I::Im8)
        .add_im8(12)
        // push 34
        .add_ins(I::Im8)
        .add_im8(34)
        // 12 - 34
        .add_ins(I::Sub)
        // - (12 - 34)
        .add_ins(I::Neg)
        // alloc 8 bytes
        .add_ins(I::Alloc)
        .add_im8(8)
        // push addr = 0, store it to addr
        .add_ins(I::Im8)
        .add_im8(0)
        .add_ins(I::Store64)
        // push 3
        .add_ins(I::Im8)
        .add_im8(3)
        // push addr = 0, load from addr
        .add_ins(I::Im8)
        .add_im8(0)
        .add_ins(I::Load64)
        // dealloc 8 bytes
        .add_ins(I::Dealloc)
        .add_im8(8)
        // add them
        .add_ins(I::Add);

    println!("{ins:?}");
    let p = ProgramMemory::new(ins, Vec::new());
    let mut m = Machine::new();
    while m.state() != MachineState::Ended {
        println!("{m:?}"); // print the state before fetching instruction
        m.run_program(&p); // fetch instruction, run and increase pc
    }
    println!("{m:?}"); // print end state
}
