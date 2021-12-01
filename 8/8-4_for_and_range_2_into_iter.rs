fn main() {
  let names = vec!["Bob", "Frank", "Ferris"];

  for name in names.into_iter() {
      match name {
          "Ferris" => println!("There is a rustacean among us!"),
          _ => println!("Hello {}", name),
      }
  }
  // print!("{:?}", names)
  // 再利用しようとすると、以下エラーが出てくる。
  // move occurs because `names` has type `Vec<&str>`, which does not implement the `Copy` trait
  // `names` moved due to this method call
  // value borrowed here after move

  // into_iterはバージョンによって動作が異なったりする様子。
  // https://doc.rust-lang.org/edition-guide/rust-2021/IntoIterator-for-arrays.html
}