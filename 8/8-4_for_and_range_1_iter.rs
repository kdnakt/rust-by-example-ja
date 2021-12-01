fn main() {
  let names = vec!["Bob", "Frank", "Ferris"];

  for name in names.iter() {
      match name {
          &"Ferris" => println!("There is a rustacean among us!"),
          _ => println!("Hello {}", name),
      }
  }
  // 再利用する事が出来る
  print!("{:?}", names)
}