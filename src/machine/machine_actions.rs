use super::program_memory::ProgramMemory;
use super::Machine;
use super::MachineState;
use crate::instruction::Instructions as I;

impl Machine {
    pub fn run_program(&mut self, program: &ProgramMemory) {
        let instruct = program.get_opcode_at(self.pc);
        match instruct {
            I::Nop => {}
            I::Interupt => self.state = MachineState::Interupted,
            I::FromR => {
                let t = self.r_pop();
                self.push(t);
            }
            I::ToR => {
                let t = self.pop();
                self.r_push(t);
            }
            I::Swap => self.calculation_stack.swap(),
            I::Over => self.calculation_stack.over(),
            I::Dup => self.calculation_stack.dup(),
            I::Discard => self.calculation_stack.discard(),
            I::Im8 => {
                let im = program.get_im_u8_at(self.pc + 1);
                self.push(im as u64);
                self.skip_im(size_of::<u8>()); // pc + 8
            }
            I::Im16 => {
                let im = program.get_im_u16_at(self.pc + 1);
                self.push(im as u64);
                self.skip_im(size_of::<u16>()); // pc + 8
            }
            I::Im32 => {
                let im = program.get_im_u32_at(self.pc + 1);
                self.push(im as u64);
                self.skip_im(size_of::<u32>()); // pc + 8
            }
            I::Im64 => {
                let im = program.get_im_u64_at(self.pc + 1);
                self.push(im);
                self.skip_im(size_of::<u64>()); // pc + 8
            }
            I::Store8 => {
                let addr = self.pop();
                let value = self.pop();
                self.runtime_memory.local_save_u8(value as u8, addr);
            }
            I::Load8 => {
                let addr = self.pop();
                let value = self.runtime_memory.local_get_u8(addr);
                self.push(value as u64);
            }
            I::Store16 => {
                let addr = self.pop();
                let value = self.pop();
                self.runtime_memory.local_save_u16(value as u16, addr);
            }
            I::Load16 => {
                let addr = self.pop();
                let value = self.runtime_memory.local_get_u16(addr);
                self.push(value as u64);
            }
            I::Store32 => {
                let addr = self.pop();
                let value = self.pop();
                self.runtime_memory.local_save_u32(value as u32, addr);
            }
            I::Load32 => {
                let addr = self.pop();
                let value = self.runtime_memory.local_get_u32(addr);
                self.push(value as u64);
            }
            I::Store64 => {
                let addr = self.pop();
                let value = self.pop();
                self.runtime_memory.local_save_u64(value, addr);
            }
            I::Load64 => {
                let addr = self.pop();
                let value = self.runtime_memory.local_get_u64(addr);
                self.push(value);
            }
            I::Alloc => {
                let siz = program.get_im_u8_at(self.pc + 1);
                self.runtime_memory.alloc(siz as u64);
                self.skip_im(size_of::<u8>());
            }
            I::Dealloc => {
                let siz = program.get_im_u8_at(self.pc + 1);
                self.runtime_memory.dealloc(siz as u64);
                self.skip_im(size_of::<u8>());
            }
            I::LoadData8 => {
                let addr = self.pop();
                let value = program.get_data_u8(addr);
                self.push(value as u64);
            }
            I::LoadData16 => {
                let addr = self.pop();
                let value = program.get_data_u16(addr);
                self.push(value as u64);
            }
            I::LoadData32 => {
                let addr = self.pop();
                let value = program.get_data_u32(addr);
                self.push(value as u64);
            }
            I::LoadData64 => {
                let addr = self.pop();
                let value = program.get_data_u64_at(addr);
                self.push(value);
            }
            I::J => {
                let addr = program.get_im_u64_at(self.pc + 1);
                self.pc = addr - 1; // -1 for later increase
            }
            I::Jz => {
                let addr = program.get_im_u64_at(self.pc + 1);
                let a = self.pop();
                match a {
                    0 => self.pc = addr - 1,
                    _ => (),
                }
            }
            I::Jnz => {
                let addr = program.get_im_u64_at(self.pc + 1);
                let a = self.pop();
                match a {
                    0 => (),
                    _ => self.pc = addr - 1,
                }
            }
            I::Ja => {
                let addr = self.pop();
                self.pc = addr - 1;
            }
            I::Add => {
                let a = self.pop_signed();
                let b = self.pop_signed();
                self.push_signed(a + b);
            }
            I::Addu => {
                let a = self.pop();
                let b = self.pop();
                self.push(a + b);
            }
            I::Sub => {
                let a = self.pop_signed();
                let b = self.pop_signed();
                self.push_signed(b - a);
            }
            I::Subu => {
                let a = self.pop();
                let b = self.pop();
                self.push(b - a);
            }
            I::Mul => {
                let a = self.pop_signed();
                let b = self.pop_signed();
                self.push_signed(a * b);
            }
            I::Mulu => {
                let a = self.pop();
                let b = self.pop();
                self.push(a * b);
            }
            I::Div => {
                let a = self.pop_signed();
                let b = self.pop_signed();
                self.push_signed(a / b);
            }
            I::Divu => {
                let a = self.pop();
                let b = self.pop();
                self.push(a / b);
            }
            I::Mod => {
                let a = self.pop_signed();
                let b = self.pop_signed();
                self.push_signed(a % b);
            }
            I::Modu => {
                let a = self.pop();
                let b = self.pop();
                self.push(a % b);
            }
            I::Neg => {
                let t = self.pop_signed();
                self.push_signed(-t);
            }
            I::Shl => {
                let a = self.pop();
                let b = self.pop();
                self.push(a << b);
            }
            I::Shlr => {
                let a = self.pop();
                let b = self.pop();
                self.push(a >> b);
            }
            I::Shar => {
                let a = self.pop_signed();
                let b = self.pop();
                self.push_signed(a >> b);
            }
            I::PopCnt => {
                let t = self.pop();
                self.push(t.count_ones() as u64);
            }
            I::Eq => {
                let a = self.pop();
                let b = self.pop();
                self.push((a == b) as u64);
            }
            I::Neq => {
                let a = self.pop();
                let b = self.pop();
                self.push((a != b) as u64);
            }
            I::Lt => {
                let a = self.pop_signed();
                let b = self.pop_signed();
                self.push((a < b) as u64);
            }
            I::Ltu => {
                let a = self.pop();
                let b = self.pop();
                self.push((a < b) as u64);
            }
            I::Leq => {
                let a = self.pop_signed();
                let b = self.pop_signed();
                self.push((a <= b) as u64);
            }
            I::Lequ => {
                let a = self.pop();
                let b = self.pop();
                self.push((a <= b) as u64);
            }
            I::Gt => {
                let a = self.pop_signed();
                let b = self.pop_signed();
                self.push((a > b) as u64);
            }
            I::Gtu => {
                let a = self.pop();
                let b = self.pop();
                self.push((a > b) as u64);
            }
            I::Geq => {
                let a = self.pop_signed();
                let b = self.pop_signed();
                self.push((a >= b) as u64);
            }
            I::Gequ => {
                let a = self.pop();
                let b = self.pop();
                self.push((a >= b) as u64);
            }
            I::Addf => {
                let a = f64::from_bits(self.pop());
                let b = f64::from_bits(self.pop());
                self.push((a + b).to_bits());
            }
            I::Subf => {
                let a = f64::from_bits(self.pop());
                let b = f64::from_bits(self.pop());
                self.push((b - a).to_bits());
            }
            I::Mulf => {
                let a = f64::from_bits(self.pop());
                let b = f64::from_bits(self.pop());
                self.push((a * b).to_bits());
            }
            I::Divf => {
                let a = f64::from_bits(self.pop());
                let b = f64::from_bits(self.pop());
                self.push((a / b).to_bits());
            }
            I::Modf => {
                let a = f64::from_bits(self.pop());
                let b = f64::from_bits(self.pop());
                self.push((a % b).to_bits());
            }
            I::Negf => {
                let a = f64::from_bits(self.pop());
                self.push((-a).to_bits());
            }
            I::Invf => {
                let a = f64::from_bits(self.pop());
                self.push((1. / a).to_bits());
            }
            I::Sqrf => {
                let a = f64::from_bits(self.pop());
                self.push(a.sqrt().to_bits());
            }
            I::Powf => {
                let a = f64::from_bits(self.pop());
                let b = f64::from_bits(self.pop());
                self.push(a.powf(b).to_bits());
            }
            I::Expf => {
                let a = f64::from_bits(self.pop());
                self.push(a.exp().to_bits());
            }
            I::Logf => {
                let a = f64::from_bits(self.pop());
                self.push((a - 1.).ln_1p().to_bits());
            }
            I::Sinf => {
                let a = f64::from_bits(self.pop());
                self.push(a.sin().to_bits());
            }
            I::Cosf => {
                let a = f64::from_bits(self.pop());
                self.push(a.cos().to_bits());
            }
            I::Tanf => {
                let a = f64::from_bits(self.pop());
                self.push(a.tan().to_bits());
            }
            I::ArcSinf => {
                let a = f64::from_bits(self.pop());
                self.push(a.asin().to_bits());
            }
            I::ArcCosf => {
                let a = f64::from_bits(self.pop());
                self.push(a.acos().to_bits());
            }
            I::ArcTanf => {
                let a = f64::from_bits(self.pop());
                self.push(a.atan().to_bits());
            }
            I::Eqf => {
                let a = f64::from_bits(self.pop());
                let b = f64::from_bits(self.pop());
                self.push((a == b) as u64);
            }
            I::Neqf => {
                let a = f64::from_bits(self.pop());
                let b = f64::from_bits(self.pop());
                self.push((a != b) as u64);
            }
            I::Ltf => {
                let a = f64::from_bits(self.pop());
                let b = f64::from_bits(self.pop());
                self.push((a < b) as u64);
            }
            I::Leqf => {
                let a = f64::from_bits(self.pop());
                let b = f64::from_bits(self.pop());
                self.push((a <= b) as u64);
            }
            I::Gtf => {
                let a = f64::from_bits(self.pop());
                let b = f64::from_bits(self.pop());
                self.push((a > b) as u64);
            }
            I::Geqf => {
                let a = f64::from_bits(self.pop());
                let b = f64::from_bits(self.pop());
                self.push((a >= b) as u64);
            }
            I::ItoF => {
                let a = self.pop() as i64;
                self.push((a as f64).to_bits());
            }
            I::FtoI => {
                let a = f64::from_bits(self.pop());
                self.push(u64::from_le_bytes((a as i64).to_le_bytes()));
            }
            I::Sinhf => {
                let a = f64::from_bits(self.pop());
                self.push(a.sinh().to_bits());
            }
            I::Coshf => {
                let a = f64::from_bits(self.pop());
                self.push(a.cosh().to_bits());
            }
            I::Tanhf => {
                let a = f64::from_bits(self.pop());
                self.push(a.tanh().to_bits());
            }
            I::ArcSinhf => {
                let a = f64::from_bits(self.pop());
                self.push(a.asinh().to_bits());
            }
            I::ArcCoshf => {
                let a = f64::from_bits(self.pop());
                self.push(a.acosh().to_bits());
            }
            I::ArcTanhf => {
                let a = f64::from_bits(self.pop());
                self.push(a.atanh().to_bits());
            }
        };
        self.next();
        if self.pc >= program.prog_len() as u64 {
            self.state = MachineState::Ended
        }
    }
}
