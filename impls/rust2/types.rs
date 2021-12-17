pub trait MType {
    // Only to be used for MalLists
    fn push(&mut self, thing: MalType);
    fn len(&self) -> usize;
}

#[derive(Debug)]
pub enum MalType {
    Symbol(MalSymbol),
    Number(MalNumber),
    List(MalList),
}

impl MType for MalType {
    fn push(&mut self, thing: MalType) {
        match self {
            MalType::List(s) => { s.push(thing); },
            _ => { panic!("push used on something other than MalList!"); },
        }
    }

    fn len(&self) -> usize {
        match self {
            MalType::List(s) => { s.len() },
            _ => { panic!("len used on something other than MalList!"); },
        }
    }
}  

impl std::fmt::Display for MalType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MalType::Symbol(s) => { s.fmt(f) },
            MalType::Number(n) => { n.fmt(f) },
            MalType::List(l) => { l.fmt(f) },
        }
        //write!(f, "({})", self.val)
    }
}

#[derive(Debug)]
pub struct MalSymbol {
    val: String,
}

impl MalSymbol {
    pub fn new(v: &str) -> MalSymbol {
        MalSymbol {
            val: v.to_string(),
        }
    }
}

impl std::fmt::Display for MalSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.val)
    }
}

#[derive(Debug)]
pub struct MalNumber {
    val: i64,
}

impl MalNumber {
    pub fn new(v: i64) -> MalNumber {
        MalNumber {
            val: v,
        }
    }
}

impl std::fmt::Display for MalNumber{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.val)
    }
}

#[derive(Debug)]
pub struct MalList {
    val: Vec<MalType>,
}

impl MalList {
    pub fn new() -> MalList {
        MalList {
            val: Vec::<MalType>::new(),
        }
    }
    pub fn push(&mut self, m: MalType) {
        self.val.push(m);
    }

    pub fn len(&self) -> usize {
        self.val.len()
    }
}

impl std::fmt::Display for MalList{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // if self.val.len() == 0 {
        //     write!(f, "( )")?;
        //     return Ok(());
        // }
        write!(f, "(")?;
        for v in &self.val {
            write!(f, "{}", &v)?;
        }
        write!(f, ")")?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_types() {
        let thing1 = MalType::Number(MalNumber::new(5));
        let thing2 = MalType::Symbol(MalSymbol::new("+"));
        let mut thing3: MalType = MalType::List(MalList::new());

        thing3.push(thing2);
        thing3.push(thing1);

        println!("{:?}", thing3);
    }
}