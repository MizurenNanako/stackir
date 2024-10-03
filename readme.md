# StackIR for learning

Library for a simple stack-based IR with its interpreter.

## Model

An StackIR interpreter consists of:

- Program Counter: an integer.
- State Indicater of `Running`, `Interupted`, `Ended`
- Calculation Stack: the stack for computation.
- Return Stack: the stack used to handle call and return.
- Runtime Memory: a linearal memory, supports random access with bounds check.

An StackIR Program consists of:

- Program: sequence of the IR instructions.
- Data: the static immutable global data.

## Instructions

Most of the instructions are with 1 byte length.
Excepts for `Alloc`, `Dealloc`, `Im8` takes 2 bytes, `Im16` takes 3 bytes, `Im32` takes 5 bytes and `Im64` takes 9 bytes.

- Utilities
  - Nop: do nothing.
  - Interupt:
- Stack manipulation
  - FromR
  - ToR
  - Swap
  - Over
  - Dup
  - Discard
  - Im8
  - Im16
  - Im32
  - Im64
- Memory manipulation
  - Store64
  - Load64
  - Alloc
  - Dealloc
  - LoadData64
- Branch
  - J
  - Jz
  - Jnz
  - Ja
- Arithmatic (i64)
  - Add
  - Addu
  - Sub
  - Subu
  - Mul
  - Mulu
  - Div
  - Divu
  - Mod
  - Modu
  - Neg
  - Shl
  - Shlr
  - Shar
  - PopCnt
- Comparaion (i64)
  - Eq
  - Neq
  - Lt
  - Ltu
  - Leq
  - Lequ
  - Gt
  - Gtu
  - Geq
  - Gequ
- Floating (f64)
  - Addf
  - Subf
  - Mulf
  - Divf
  - Modf
  - Negf
  - Invf
  - Sqrf
  - Powf
  - Expf
  - Logf
  - Sinf
  - Cosf
  - Tanf
  - ArcSinf
  - ArcCosf
  - ArcTanf
  - Sinhf
  - Coshf
  - Tanhf
  - ArcSinhf
  - ArcCoshf
  - ArcTanhf
- Comparaion (f64)
  - Eqf
  - Neqf
  - Ltf
  - Leqf
  - Gtf
  - Geqf
- Conversion
  - ItoF
  - FtoI
