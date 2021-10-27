fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    // copy `an_integer` into `copied_integer`
    // `an_integer`を`copied_integer`へとコピー
    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // The compiler warns about unused variable bindings; these warnings can
    // be silenced by prefixing the variable name with an underscore
    // 使用されていない変数があると、コンパイラは警告を出します。
    // 変数名の頭に`_`（アンダーバー）を付けると警告を消すことができます。
    let _unused_variable = 3u32;

    let _noisy_unused_variable = 2u32;
    // FIXME ^ Prefix with an underscore to suppress the warning
    // FIXME ^ 頭にアンダーバーを付けて、警告を抑えましょう。

    // https://doc.rust-jp.rs/rust-by-example-ja/variable_bindings.html の実行では警告が出ない。
    // https://play.rust-lang.org/ で実行するとちゃんと警告が出る。
}