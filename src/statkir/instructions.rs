use num_enum::{IntoPrimitive, TryFromPrimitive};

#[allow(dead_code)]
#[repr(u8)]
#[derive(Debug, Eq, PartialEq, Clone, TryFromPrimitive, IntoPrimitive)]
pub enum Instructions {
    Nop = 0,
    Interupt = 1,
    // stack manipulation
    FromR = 2,
    Swap = 3,
    Over = 4,
    Dup = 5,
    Discard = 6,
    Imme = 7,
    // Memory manipulation
    Store = 8,
    Load = 9,
    // Branch
    J = 10,
    Jz = 11,
    Jnz = 12,
    // Computed Branch
    Ja = 13,
    // arithmatic (i64)
    Add = 14,
    Addu = 15,
    Sub = 16,
    Subu = 17,
    Mul = 18,
    Mulu = 19,
    Div = 20,
    Divu = 21,
    Mod = 22,
    Modu = 23,
    Neg = 24,
    Shl = 25,
    Shlr = 26,
    Shar = 27,
    PopCnt = 28,
    // comparaion (i64)
    Eq = 29,
    Neq = 30,
    Lt = 31,
    Ltu = 32,
    Leq = 33,
    Lequ = 34,
    Gt = 35,
    Gtu = 36,
    Geq = 37,
    Gequ = 38,
    // floating (f64)
    Addf = 39,
    Subf = 40,
    Mulf = 41,
    Divf = 42,
    Modf = 43,
    Negf = 44,
    Invf = 45,
    Sqrf = 46,
    Powf = 47,
    Expf = 48,
    Logf = 49,
    Sinf = 50,
    Cosf = 51,
    Tanf = 52,
    ArcSinf = 53,
    ArcCosf = 54,
    ArcTanf = 55,
    // comparaion (f64)
    Eqf = 56,
    Neqf = 57,
    Ltf = 58,
    Leqf = 59,
    Gtf = 60,
    Geqf = 61,
    // conversion
    ItoF = 62,
    FtoI = 63,
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
