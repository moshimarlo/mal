use std::fmt;

pub enum MalType {
    MalSymbol(MalSymbol),
    MalNumber(MalNumber),
    MalList(MalList),
}

pub struct MalSymbol {
    val: String,
}

impl fmt::Display for MalSymbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.val)
    }
}

pub struct MalNumber {
    val: i64,
}

impl fmt::Display for MalNumber{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.val)
    }
}

pub struct MalList {
    val: Vec<MalType>,
}

impl MalList {
    pub fn new() -> MalList {
        MalList {
            val: Vec::<MalType>::new(),
        }
    }
}

impl fmt::Display for MalList{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?})", self.val)
    }
}

#[cfg(test)]
fn main() {
    let thing: MalType;
}
