fn main() {
    // Increment via closures and functions.
    // 関数とクロージャのそれぞれで数値をインクリメントする
    fn function(i: i32) -> i32 { i + 1 }

    // Closures are anonymous, here we are binding them to references
    // Annotation is identical to function annotation but is optional
    // as are the `{}` wrapping the body. These nameless functions
    // are assigned to appropriately named variables.
    // 型アノテーションは、通常の関数と同様の方法で行えるが、必須ではない。
    // `{}`も必須ではない。
    // クロージャは一種の無名関数なので、適切な変数にバインディングしてやるとよい
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred  = |i     |          i + 1  ;
    let closure_inferred2 = |i     |        { i + 1 };

    let i = 1;
    // Call the function and closures.
    // 関数とクロージャを呼び出す。
    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));
    println!("closure_inferred2: {}", closure_inferred2(i));

    // A closure taking no arguments which returns an `i32`.
    // The return type is inferred.
    // 引数なしで`i32`を返すクロージャ。
    // 戻り値の型は推論された。
    let one = || 1;
    println!("closure returning one: {}", one());
    let two = || { 2 };
    println!("closure returning two: {}", two());
}
