use stackir::instruction::Instructions as I;
use stackir::machine::program_memory::ProgramMemory;
use stackir::machine::{Machine, MachineState};

fn main() {
    use stackir::instruction::program_maker::ProgramMaker;
    let mut ins = Vec::new();
    ins
        // jump to 1+8+1=10
        .add_ins(I::J) // 0
        .add_im64(10) // 1
        // interupt
        .add_ins(I::Interupt) // 9
        // push 12
        .add_ins(I::Im8) // 10
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
        .add_im8(2)
        // push addr = 0, store it to addr
        .add_ins(I::Im8)
        .add_im8(0)
        .add_ins(I::Store16)
        // push 3
        .add_ins(I::Im8)
        .add_im8(3)
        // push addr = 0, load from addr
        .add_ins(I::Im8)
        .add_im8(0)
        .add_ins(I::Load16)
        // add them
        .add_ins(I::Add)
        .add_ins(I::Im16)
        .add_im16(257)
        .add_ins(I::Dup)
        // alloc 8 bytes
        .add_ins(I::Alloc)
        .add_im8(1)
        // store
        .add_ins(I::Im8)
        .add_im8(2)
        .add_ins(I::Store8)
        // dealloc 3 bytes
        .add_ins(I::Dealloc)
        .add_im8(3)
        // jump to 9, interupt the program
        .add_ins(I::Im8)
        .add_im8(9)
        .add_ins(I::Ja);

    println!("{ins:?}");
    let p = ProgramMemory::new(ins, Vec::new());
    let mut m = Machine::new();
    while let MachineState::Running = m.state() {
        println!("{m:?}"); // print the state before fetching instruction
        m.run_program(&p); // fetch instruction, run and increase pc
    }
    println!("{m:?}"); // print end state
}
