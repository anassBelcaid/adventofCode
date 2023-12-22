use regex::Regex;
pub struct Computer {
    registers: [i32; 4], // value of the four register
}

pub enum Operation {
    COPY(String, usize), // copy instruction from a source athat could be a value or a register
    INCR(usize),         // increase register index usize
    DECR(usize),         // decrease teh register indexed by usize
    JNZ(usize, i32),     // relative jup but if the register at [index] is not zero
}
