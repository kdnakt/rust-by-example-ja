// `test!` will compare `$left` and `$right`
// in different ways depending on how you invoke it:
// `test!`は`$left`と`$right`を異なる呼び出し方に応じて
// 比較する。
macro_rules! test {
    // Arguments don't need to be separated by a comma.
    // Any template can be used!
    // 引数はカンマで区切らなくてもよい
    // テンプレートの形態は自由！
    ($left:expr; and $right:expr) => {
        println!("{:?} and {:?} is {:?}",
                 stringify!($left),
                 stringify!($right),
                 $left && $right)
    };
    // ^ each arm must end with a semicolon.
    // それぞれの`=>`節はセミコロンで終わる必要がある。
    ($left:expr; or $right:expr) => {
        println!("{:?} or {:?} is {:?}",
                 stringify!($left),
                 stringify!($right),
                 $left || $right)
    };
    ($left:expr; xor $right:expr) => {
        println!("{:?} xor {:?} is {:?}",
                 stringify!($left),
                 stringify!($right),
                 $left ^ $right)
    };
}

fn main() {
    test!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
    test!(true; or false);
    test!(true; xor false);
    test!(true; xor true);
    // test!(true; nand true);
    // error: no rules expected the token `nand`
}