macro_rules! create_function {
    // This macro takes an argument of designator `ident` and
    // creates a function named `$func_name`.
    // The `ident` designator is used for variable/function names.
    // このマクロは`ident`識別子に対応する値を引数として取り
    // `$func_name`という名の関数を作成する。
    // `ident`識別子は関数・変数の名前用の識別子である。
    ($func_name:ident) => {
        fn $func_name() {
            // The `stringify!` macro converts an `ident` into a string.
            // `stringify!`というマクロは`ident`を文字列に変える。
            println!("You called {:?}()",
                     stringify!($func_name));
        }
    };
}

// Create functions named `foo` and `bar` with the above macro.
// 上のマクロを利用して`foo`、`bar`という名の関数を作成する。
create_function!(foo);
create_function!(bar);

macro_rules! print_result {
    // This macro takes an expression of type `expr` and prints
    // it as a string along with its result.
    // The `expr` designator is used for expressions.
    // このマクロは`expr`識別子に対応する値を引数として取り、
    // その結果を文字列としてプリントする。
    // `expr`識別子は式に対応する。
    ($expression:expr) => {
        // `stringify!` will convert the expression *as it is* into a string.
        // `stringify!`は式を *そのままの形で* 文字列に変換する
        println!("{:?} = {:?}",
                 stringify!($expression),
                 $expression);
    };
}

fn main() {
    foo();
    bar();

    print_result!(1u32 + 1);

    // Recall that blocks are expressions too!
    // ブロックも式の一種であることを思い出しましょう!
    print_result!({
        let x = 1u32;

        x * x + 2 * x - 1
    });
}
