#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct ReturnStack {
    raw: Vec<u64>,
}

#[allow(dead_code)]
impl ReturnStack {
    pub(crate) fn new() -> ReturnStack {
        ReturnStack { raw: Vec::new() }
    }
    pub(crate) fn push(&mut self, v: u64) {
        self.raw.push(v)
    }
    pub(crate) fn pop(&mut self) -> u64 {
        self.raw.pop().expect("return stack underflowed.")
    }
}
