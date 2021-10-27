fn main() {
  // Suffixed literals, their types are known at initialization
  // サフィックスを指定したリテラル。型は初期化とともに確定する。
  let x = 1u8;
  let y = 2u32;
  let z = 3f32;

  // Unsuffixed literal, their types depend on how they are used
  // サフィックスを指定しないリテラル。型は使用方法に依存する。
  let i = 1;
  let f = 1.0;
  // let xf = i * f; error[E0277]: cannot multiply `{integer}` by `{float}` エラーが出る
  // zがf32なので、fもそれに呼応してf32になる。
  let f2 = z * f;
  
  // 基本的に整数型と浮動小数点は相容れないが、キャスティングすると大丈夫
  let f3:f32 = i as f32;

  // f32で明示的に指定したものはf64には代入できない。明示してない場合は変更される。
  let f3a = 2.0;
  let f4:f64 = f3a;

  // intoでも変換できる。通常intoを使って型の整合性を取る。
  let f5:f64 = f3.into();
  let f6:f64 = i.into();

  // `size_of_val` returns the size of a variable in bytes
  // `size_of_val` 関数は変数のサイズをバイトで返す。
  println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
  println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
  println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
  println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
  println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
  //println!("size of `xf` in bytes: {}", std::mem::size_of_val(&xf));
  println!("size of `f2` in bytes: {}", std::mem::size_of_val(&f2));
}