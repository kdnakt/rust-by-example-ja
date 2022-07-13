use std::marker::PhantomData;

// Make a constant with `'static` lifetime.
// `'static`ライフタイムを持つ定数を作成
static NUM: i32 = 18;

// Returns a reference to `NUM` where its `'static`
// lifetime is coerced to that of the input argument.
// `NUM`への参照を返す。ライフタイムは`'static`から引数の
// ライフタイムへと圧縮されている。
fn coerce_static<'a>(_: PhantomData<&'a i32>) -> &'a i32 {
    &NUM
}

fn main() {
    {
        // Make a `string` literal and print it:
        // 文字列リテラルを用いて変数を作成し、プリントする
        let static_string: &'static str = "I'm in read-only memory";
        let not_static_string: String = static_string.to_string();
        println!("not_static_string: {}", not_static_string);

        // When `static_string` goes out of scope, the reference
        // can no longer be used, but the data remains in the binary.
        // `static_string`がスコープから抜けると、参照は使用することが
        // できなくなるが、データはバイナリ中に残る。
    }

    {
        // Make an integer to use for `coerce_static`:
        // `coerce_static`関数を呼び出すために、整数を作成
        let lifetime_phantom = PhantomData;

        // Coerce `NUM` to lifetime of `lifetime_num`:
        // `NUM`を`lifetime_num`のライフタイムへと圧縮
        let coerced_static = coerce_static(lifetime_phantom);

        println!("coerced_static: {}", coerced_static);
    }

    println!("NUM: {} stays accessible!", NUM);
}
