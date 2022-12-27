macro_rules! calculate {
    (eval2 $e:expr) => {
        let val: usize = $e; // Force types to be integers
                                 // 型を整数に制約
        println!("{} = {}", stringify!{$e}, val);
    };
}

fn main() {
    calculate! {
        eval2 1 + 2 // hehehe `eval` is _not_ a Rust keyword!
                   // `eval`はRustのキーワード *じゃない* よね！
    }

    calculate! {
        eval2 (1 + 2) * (3 / 4)
    }
}
