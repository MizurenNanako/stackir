use stackir::statkir::{
    instructions::Instructions as I,
    machine::{Machine, MachineState, ProgramMemory},
};

fn im_u64(im: u64) -> Vec<u8> {
    let a = im.to_le_bytes();
    vec![a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7]]
}

fn main() {
    let mut ins = Vec::new();
    ins.push(I::Imme.opcode());
    ins.append(&mut im_u64(123));
    ins.push(I::Imme.opcode());
    ins.append(&mut im_u64(2));
    ins.push(I::Imme.opcode());
    ins.append(&mut im_u64(4567));
    ins.push(I::Mul.opcode());
    ins.push(I::Add.opcode());

    println!("{ins:?}");
    let p = ProgramMemory::new(ins, Vec::new());
    let mut m = Machine::new();
    while m.state() != MachineState::Ended {
        println!("{m:?}");
        m.run_program(&p);
    }
    println!("{m:?}");
}
