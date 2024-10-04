use crate::machine::Instructions;
use std::io::Read;

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
    pub fn prog_len(&self) -> usize {
        self.prog.len()
    }
    pub(crate) fn get_opcode_at(&self, index: u64) -> Instructions {
        let opcode = self.prog[index as usize];
        Instructions::of_opcode(opcode)
            .expect(format!("get_opcode error: unsupported opcode {opcode}").as_str())
    }
    pub(crate) fn get_im_u8_at(&self, index: u64) -> u8 {
        self.prog[index as usize]
    }
    pub(crate) fn get_im_u16_at(&self, index: u64) -> u16 {
        let mut buf: [u8; 2] = [0; 2];
        let mut bytes = &self.prog[(index as usize)..(index as usize + size_of::<u16>())];
        bytes.read_exact(&mut buf[..size_of::<u16>()]).expect(
            format!("corrupted program memory at {index} read when getting immediate").as_str(),
        );
        u16::from_le_bytes(buf)
    }
    pub(crate) fn get_im_u32_at(&self, index: u64) -> u32 {
        let mut buf: [u8; 4] = [0; 4];
        let mut bytes = &self.prog[(index as usize)..(index as usize + size_of::<u32>())];
        bytes.read_exact(&mut buf[..size_of::<u32>()]).expect(
            format!("corrupted program memory at {index} read when getting immediate").as_str(),
        );
        u32::from_le_bytes(buf)
    }
    pub(crate) fn get_im_u64_at(&self, index: u64) -> u64 {
        let mut buf: [u8; 8] = [0; 8];
        let mut bytes = &self.prog[(index as usize)..(index as usize + size_of::<u64>())];
        bytes.read_exact(&mut buf[..size_of::<u64>()]).expect(
            format!("corrupted program memory at {index} read when getting immediate").as_str(),
        );
        u64::from_le_bytes(buf)
    }
    pub(crate) fn get_data_u8(&self, index: u64) -> u8 {
        self.data
            .get(index as usize)
            .expect(
                format!("corrupted program memory at {index} read when getting immediate").as_str(),
            )
            .clone()
    }
    pub(crate) fn get_data_u16(&self, index: u64) -> u16 {
        let mut buf: [u8; 2] = [0; 2];
        let mut bytes = &self.data[(index as usize)..(index as usize + size_of::<u16>())];
        bytes.read_exact(&mut buf[..size_of::<u16>()]).expect(
            format!("corrupted program memory at {index} read when getting immediate").as_str(),
        );
        u16::from_le_bytes(buf)
    }
    pub(crate) fn get_data_u32(&self, index: u64) -> u32 {
        let mut buf: [u8; 4] = [0; 4];
        let mut bytes = &self.data[(index as usize)..(index as usize + size_of::<u32>())];
        bytes.read_exact(&mut buf[..size_of::<u32>()]).expect(
            format!("corrupted program memory at {index} read when getting immediate").as_str(),
        );
        u32::from_le_bytes(buf)
    }
    pub(crate) fn get_data_u64_at(&self, index: u64) -> u64 {
        let mut buf: [u8; 8] = [0; 8];
        let mut bytes = &self.data[(index as usize)..(index as usize + size_of::<u64>())];
        bytes.read_exact(&mut buf[..size_of::<u64>()]).expect(
            format!("corrupted program memory at {index} read when getting immediate").as_str(),
        );
        u64::from_le_bytes(buf)
    }
}
