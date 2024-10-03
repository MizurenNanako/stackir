use num_enum::{IntoPrimitive, TryFromPrimitive};

#[allow(dead_code)]
#[repr(u8)]
#[derive(Debug, Eq, PartialEq, Clone, TryFromPrimitive, IntoPrimitive)]
pub enum Instructions {
    Nop = 0,
    Interupt,
    // stack manipulation
    FromR,
    ToR,
    Swap,
    Over,
    Dup,
    Discard,
    Im8,
    Im16,
    Im32,
    Im64,
    // Memory manipulation
    Store64,
    Load64,
    Alloc,
    Dealloc,
    LoadData64,
    // Branch
    J,
    Jz,
    Jnz,
    // Computed Branch
    Ja,
    // arithmatic (i64)
    Add,
    Addu,
    Sub,
    Subu,
    Mul,
    Mulu,
    Div,
    Divu,
    Mod,
    Modu,
    Neg,
    Shl,
    Shlr,
    Shar,
    PopCnt,
    // comparaion (i64)
    Eq,
    Neq,
    Lt,
    Ltu,
    Leq,
    Lequ,
    Gt,
    Gtu,
    Geq,
    Gequ,
    // floating (f64)
    Addf,
    Subf,
    Mulf,
    Divf,
    Modf,
    Negf,
    Invf,
    Sqrf,
    Powf,
    Expf,
    Logf,
    Sinf,
    Cosf,
    Tanf,
    ArcSinf,
    ArcCosf,
    ArcTanf,
    Sinhf,
    Coshf,
    Tanhf,
    ArcSinhf,
    ArcCoshf,
    ArcTanhf,
    // comparaion (f64)
    Eqf,
    Neqf,
    Ltf,
    Leqf,
    Gtf,
    Geqf,
    // conversion
    ItoF,
    FtoI,
}

#[allow(dead_code)]
impl Instructions {
    pub fn opcode(&self) -> u8 {
        self.clone() as u8
    }
    pub fn of_opcode(opcode: u8) -> Option<Self> {
        match Self::try_from_primitive(opcode) {
            Ok(a) => Some(a),
            Err(_) => None,
        }
    }
}

#[allow(dead_code)]
pub trait ProgramMaker {
    fn add_ins(&mut self, ins: Instructions) -> &mut Self;
    fn add_im8(&mut self, im: u8) -> &mut Self;
    fn add_im16(&mut self, im: u16) -> &mut Self;
    fn add_im32(&mut self, im: u32) -> &mut Self;
    fn add_im64(&mut self, im: u64) -> &mut Self;
}

impl ProgramMaker for Vec<u8> {
    fn add_ins(&mut self, ins: Instructions) -> &mut Self {
        self.push(ins as u8);
        self
    }
    fn add_im8(&mut self, im: u8) -> &mut Self {
        self.push(im);
        self
    }
    fn add_im16(&mut self, im: u16) -> &mut Self {
        for x in im.to_le_bytes() {
            self.push(x);
        }
        self
    }
    fn add_im32(&mut self, im: u32) -> &mut Self {
        for x in im.to_le_bytes() {
            self.push(x);
        }
        self
    }
    fn add_im64(&mut self, im: u64) -> &mut Self {
        for x in im.to_le_bytes() {
            self.push(x);
        }
        self
    }
}
