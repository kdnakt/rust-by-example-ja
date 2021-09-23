// If you use a type alias, you can refer to each enum variant via its alias.
// This might be useful if the enum's name is too long or too generic, and you want to rename it.
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

fn inspect(event: VeryVerboseEnumOfThingsToDoWithNumbers) {
    match event {
        VeryVerboseEnumOfThingsToDoWithNumbers::Add => println!("Add"),
        VeryVerboseEnumOfThingsToDoWithNumbers::Subtract => println!("Subtract"),
    }
}

// [memo] fn inspectとは別にimplでも宣言してみた
impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }

    fn toString(&self, x: i32, y: i32) -> String{
        match self {
            Self::Add => x.to_string() + &y.to_string(),
            _ => x.to_string() + "-" + &y.to_string(),
        }
    }
}

// Creates a type alias
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;


// [memo] 先にaliasを宣言して、fnでaliasからmatchしても問題ない
type Operations2 = VeryVerboseEnumOfThingsToDoWithNumbers;

fn inspect2(event: Operations2) {
    match event {
        Operations2::Add => println!("Operations2 Add"),
        Operations2::Subtract => println!("Operations2 Subtract"),
    }
}

// [memo] structについてもaliasが動作することを確認
#[derive(Debug)]
struct Person<'a> {
    // The 'a defines a lifetime
    name: &'a str,
    age: u8,
}

type Person2<'a> = Person<'a>;

fn main() {
    // We can refer to each variant via its alias, not its long and inconvenient
    // name.
    let x = Operations::Add;
    inspect(x);

    // [memo] aliasを設定したからと言って元のenumの名前で呼び出せる
    println!("{}", VeryVerboseEnumOfThingsToDoWithNumbers::Add.run(5,3));
    println!("{}", VeryVerboseEnumOfThingsToDoWithNumbers::Subtract.run(5,3));
    println!("{}", Operations::Add.run(5,3));
    println!("{}", Operations::Subtract.run(5,3));
    println!("{}", Operations::Add.toString(5,3));
    println!("{}", Operations::Subtract.toString(5,3));

    let x2 = Operations2::Add;
    inspect2(x2);

    // Create struct with field init shorthand
    let name = "Peter";
    let age = 27;
    let peter = Person2 { name, age };

    // Print debug struct
    println!("{:?}", peter);
}

// Result
// Add
// 8
// 2
// 8
// 2
// 53
// 5-3
// Operations2 Add
// Person { name: "Peter", age: 27 }