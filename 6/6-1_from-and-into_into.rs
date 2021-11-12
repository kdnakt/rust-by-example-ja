use std::convert::From;
use std::convert::Into;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        println!("From called.");
        Number { value: item }
    }
}

impl Into<i32> for Number {
    fn into(self) -> i32 {
        println!("Into called");
        self.value
    }
}

fn main() {
    let int = 5;
    // Try removing the type declaration
    let num: Number = int.into(); // 元々のi32のinto()が呼ばれてる
    println!("My number is {:?}", num);
    let int2: i32 = num.into();
    println!("int2: {}", int2);
}
