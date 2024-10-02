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
    Imme,
    // Memory manipulation
    Store,
    Load,
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
