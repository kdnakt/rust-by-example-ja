// A function which takes a closure as an argument and calls it.
// <F> denotes that F is a "Generic type parameter"
fn apply<F>(mut f: F) where
    // The closure takes no input and returns nothing.
    // クロージャには引数も返り値もない。
    F: FnMut() {
    // ^ TODO: Try changing this to `Fn` or `FnMut`.
    // ^ TODO: ここを`Fn`あるいは`FnMut`に変えてみましょう。

    f();
}

// A function which takes a closure and returns an `i32`.
// クロージャを引数に取り、`i32`を返す関数
fn apply_to_3<F: FnOnce(i32)->i32>(f: F) -> i32{
    f(3)
}

fn main() {
    use std::mem;

    let greeting = "hello";
    // A non-copy type.
    // `to_owned` creates owned data from borrowed one
    // コピーではなくmoveが起きる型
    let mut farewell = "goodbye".to_string();

    // Capture 2 variables: `greeting` by reference and
    // `farewell` by value.
    // 変数を2つ補足。`greeting`は参照を、
    // `farewell`は値をそれぞれ捕捉する。
    let diary = || {
        // `greeting` is by reference: requires `Fn`.
        // `greeting`は参照なので、`Fn`が必要。
        println!("I said {}.", greeting);

        // Mutation forces `farewell` to be captured by
        // mutable reference. Now requires `FnMut`.
        // `farewell`の値を変更するので、この時点で`FnMut`
        // が必要になる。
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // Manually calling drop forces `farewell` to
        // be captured by value. Now requires `FnOnce`.
        // `mem::drop`を明示的に呼ぶと`farewell`が値で
        // 捕捉される必要性が発生する。よって`FnOnce`が必要になる。
        //mem::drop(farewell);
    };

    // Call the function which applies the closure.
    // クロージャを適用する関数を実行。
    apply(diary);

    // `double` satisfies `apply_to_3`'s trait bound
    let double = |x:i32| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));
}
