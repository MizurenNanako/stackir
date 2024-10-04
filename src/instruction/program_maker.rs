use crate::instruction::Instructions;

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
