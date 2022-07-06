// `print_refs` takes two references to `i32` which have different
// lifetimes `'a` and `'b`. These two lifetimes must both be at
// least as long as the function `print_refs`.
// `print_refs`は`i32`への参照を2つとり、それぞれ`'a`と`'b`という
// ライフタイムを持つ。これらのライフタイムは最短でも`print_refs`
// 関数と同じになる。
fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
  println!("x is {} and y is {}", x, y);
}

// A function which takes no arguments, but has a lifetime parameter `'a`.
// 引数を取らないがライフタイムパラメータ`'a`を持つ関数。
fn failed_borrow<'a>() {
  let _x = 12;

  // ERROR: `_x` does not live long enough
  // エラー: `_x`の寿命が短すぎる。
  // コメントアウト
  // let y: &'a i32 = &_x;
  // Attempting to use the lifetime `'a` as an explicit type annotation 
  // inside the function will fail because the lifetime of `&_x` is shorter
  // than that of `y`. A short lifetime cannot be coerced into a longer one.
  // `&_x`のライフタイムは`y`のそれよりも短いため、関数内で`'a`を使用して
  // 変数のライフタイムを指定しようとすると失敗する。つまり、短いライフタイム
  // を持つ参照をより長いものに強制的に代入することはできない。

  // 最終部で使用することでライフタイムを広げられるか実験 => 後半の説明を読み、staticになるので無理という結論
  println!("_x is {}", _x);
}

fn main() {
  // Create variables to be borrowed below.
  // 下で借用するための変数を作成
  let (four, nine) = (4, 9);
  
  // Borrows (`&`) of both variables are passed into the function.
  // 2つの変数の借用(`&`)が関数に渡される。
  print_refs(&four, &nine);
  // Any input which is borrowed must outlive the borrower. 
  // In other words, the lifetime of `four` and `nine` must 
  // be longer than that of `print_refs`.
  // 借用された変数の寿命は、借り手のそれよりも長くなくてはならない。
  // つまり、`four`、`nine`のライフタイムは`print_refs`のそれよりも
  // 長くなくてはならない。
  
  failed_borrow();
  // `failed_borrow` contains no references to force `'a` to be 
  // longer than the lifetime of the function, but `'a` is longer.
  // Because the lifetime is never constrained, it defaults to `'static`.
  // `failed_borrow`は関数のライフタイムよりも`'a`を長くさせるような
  // 参照を持たないが、それでも`'a`のほうが長くなる。なぜならそのような
  // 場合`'a`はデフォルトで`'static`になるからである。
}