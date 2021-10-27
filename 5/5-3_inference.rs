fn main() {
  // Because of the annotation, the compiler knows that `elem` has type u8.
  // アノテーションのおかげで、コンパイラは`elem`がu8型であることがわかる。
  let elem = 5u8;

  // Create an empty vector (a growable array).
  // 空のベクトル（可変長の配列）を生成
  let mut vec = Vec::new();
  // At this point the compiler doesn't know the exact type of `vec`, it
  // just knows that it's a vector of something (`Vec<_>`).
  // この時点でコンパイラは`vec`の型を知らず、
  // 単に何らかの値のベクトル(`Vec<_>`)であるということだけを把握している。

  // Insert `elem` in the vector.
  // `elem`をベクトルに挿入
  vec.push(elem);
  // Aha! Now the compiler knows that `vec` is a vector of `u8`s (`Vec<u8>`)
  // TODO ^ Try commenting out the `vec.push(elem)` line
  // よし！ これでコンパイラは`vec`が`u8`のベクトル(`Vec<u8>`)であることを把握する。
  // TODO ^ 上の `vec.push(elem)` をコメントアウトしてみましょう。

  println!("{:?}", vec);
  
  // 型が全然書かれていないと可読性が落ちるので、関数には型を書く形になった。
  // 変数での型が解らなくなるような長さになってきているなら関数に切り出すべき。
  // 機械的に変数型をつけられるツールもあるらしい。
}