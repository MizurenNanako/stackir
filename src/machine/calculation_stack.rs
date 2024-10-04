#[allow(dead_code)]
#[derive(Debug)]
pub struct CalculationStack {
    raw: Vec<u64>,
}

#[allow(dead_code)]
impl CalculationStack {
    pub(crate) fn new() -> CalculationStack {
        CalculationStack { raw: Vec::new() }
    }
    pub(crate) fn push(&mut self, v: u64) {
        self.raw.push(v)
    }
    pub(crate) fn pop(&mut self) -> u64 {
        self.raw
            .pop()
            .expect("pop(?): calculation stack underflowed.")
    }
    pub(crate) fn discard(&mut self) {
        self.raw
            .pop()
            .expect("discard(?): calculation stack underflowed.");
        ()
    }
    pub(crate) fn dup(&mut self) {
        let a = self
            .raw
            .last()
            .expect("dup(?): calculation stack underflowed.");
        self.raw.push(a.clone())
    }
    pub(crate) fn swap(&mut self) {
        let a = self
            .raw
            .pop()
            .expect("swap(?, ?): calculation stack underflowed.");
        let b = self
            .raw
            .pop()
            .expect("swap(a, ?): calculation stack underflowed.");
        self.raw.push(a);
        self.raw.push(b);
    }
    pub(crate) fn over(&mut self) {
        let a = self
            .raw
            .pop()
            .expect("over(?, ?): calculation stack underflowed.");
        let b = self
            .raw
            .pop()
            .expect("over(a, ?): calculation stack underflowed.");
        self.raw.push(b);
        self.raw.push(a);
        self.raw.push(b.clone());
    }
}
