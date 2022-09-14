// This is a simple macro named `say_hello`.
// `say_hello`という名のシンプルなマクロ
macro_rules! say_hello {
    // `()` indicates that the macro takes no argument.
    // `()`はマクロが引数をとらないことを示す。
    () => {
        // The macro will expand into the contents of this block.
        // マクロは（訳注: プリコンパイルの段階で）このブロック内の内容に展開されます。
        println!("Hello!")
    };
}

fn main() {
    // This call will expand into `println!("Hello");`
    // この呼び出しは`println!("Hello");`に置き換えられます。
    say_hello!();
}
