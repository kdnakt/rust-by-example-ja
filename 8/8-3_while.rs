fn main() {
  // A counter variable
  // カウンタとなる変数
  let mut n = 1;

  // Loop while `n` is less than 101
  // `n`が101以下である場合のループ
  while n < 101 {
      if n % 15 == 0 {
          println!("fizzbuzz");
          // break n;
          // "can only break with a value inside `loop` or breakable block"のエラーが出る。
          // 通常のbreakは出来る
      } else if n % 3 == 0 {
          println!("fizz");
      } else if n % 5 == 0 {
          println!("buzz");
      } else {
          println!("{}", n);
      }

      // Increment counter
      // カウンタに1を追加
      n += 1;
  };
}