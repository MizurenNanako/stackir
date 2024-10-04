use std::io::Read;

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct RuntimeMemory {
    // addr, value
    raw: Vec<u8>,
}

#[allow(dead_code)]
impl RuntimeMemory {
    pub(crate) fn new() -> Self {
        Self { raw: Vec::new() }
    }
    pub(crate) fn alloc(&mut self, size: u64) {
        for _ in 0..size {
            self.raw.push(0)
        }
    }
    pub(crate) fn dealloc(&mut self, size: u64) {
        for _ in 0..size {
            self.raw
                .pop()
                .expect("dealloc: runtime memory underflowed.");
        }
    }
    pub(crate) fn local_get_u8(&self, start_pos: u64) -> u8 {
        self.raw
            .get(start_pos as usize)
            .expect(
                format!(
                    "local_get_u64: accessing invaild memory location {}.",
                    start_pos
                )
                .as_str(),
            )
            .clone()
    }
    pub(crate) fn local_save_u8(&mut self, value: u8, start_pos: u64) {
        self.raw[start_pos as usize] = value;
    }
    pub(crate) fn local_get_u16(&self, start_pos: u64) -> u16 {
        let mut t: [u8; 2] = [0u8; 2];
        self.raw
            .get((start_pos as usize)..size_of::<u16>())
            .expect(
                format!(
                    "local_get_u16: accessing invaild memory location {}.",
                    start_pos
                )
                .as_str(),
            )
            .read_exact(&mut t)
            .expect("local_get_u16: invaild memory read");
        u16::from_le_bytes(t)
    }
    pub(crate) fn local_save_u16(&mut self, value: u16, start_pos: u64) {
        for (a, b) in self
            .raw
            .iter_mut()
            .skip(start_pos as usize)
            .take(size_of::<u16>())
            .zip(value.to_le_bytes())
        {
            *a = b;
        }
    }
    pub(crate) fn local_get_u32(&self, start_pos: u64) -> u32 {
        let mut t: [u8; 4] = [0u8; 4];
        self.raw
            .get((start_pos as usize)..size_of::<u32>())
            .expect(
                format!(
                    "local_get_u32: accessing invaild memory location {}.",
                    start_pos
                )
                .as_str(),
            )
            .read_exact(&mut t)
            .expect("local_get_u32: invaild memory read");
        u32::from_le_bytes(t)
    }
    pub(crate) fn local_save_u32(&mut self, value: u32, start_pos: u64) {
        for (a, b) in self
            .raw
            .iter_mut()
            .skip(start_pos as usize)
            .take(size_of::<u32>())
            .zip(value.to_le_bytes())
        {
            *a = b;
        }
    }
    pub(crate) fn local_get_u64(&self, start_pos: u64) -> u64 {
        let mut t: [u8; 8] = [0u8; 8];
        self.raw
            .get((start_pos as usize)..size_of::<u64>())
            .expect(
                format!(
                    "local_get_u64: accessing invaild memory location {}.",
                    start_pos
                )
                .as_str(),
            )
            .read_exact(&mut t)
            .expect("local_get_u64: invaild memory read");
        u64::from_le_bytes(t)
    }
    pub(crate) fn local_save_u64(&mut self, value: u64, start_pos: u64) {
        for (a, b) in self
            .raw
            .iter_mut()
            .skip(start_pos as usize)
            .take(size_of::<u64>())
            .zip(value.to_le_bytes())
        {
            *a = b;
        }
    }
}
