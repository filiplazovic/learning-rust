use std::convert::From;

#[derive(Debug)]
pub struct Number {
    pub value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}
