struct Container(i32, i32);

// A trait which checks if 2 items are stored inside of container.
// Also retrieves first or last value.
// 2つの要素がコンテナ型の中に保持されていることを確認するトレイト。
// また、最初あるいは最後の要素を取り出すこともできる。
trait Contains {
    // Define generic types here which methods will be able to utilize.
    // メソッドが使用できるジェネリック型を定義
    type A;
    type B;

    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains for Container {
    // Specify what types `A` and `B` are. If the `input` type
    // is `Container(i32, i32)`, the `output` types are determined
    // as `i32` and `i32`.
    // `A`と`B`がどの型であるかを明示。インプットの型（訳注: つまり`Self`の型）
    // が`Container(i32, i32)`である場合、出力型は`i32`と`i32`となる。
    type A = i32;
    type B = i32;

    // `&Self::A` and `&Self::B` are also valid here.
    // `&i32`の代わりに`&Self::A`または`&self::B`と書いても良い
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }
    // Grab the first number.
    // 1つ目の数を取得
    fn first(&self) -> i32 { self.0 }

    // Grab the last number.
    // 最後の数を取得
    fn last(&self) -> i32 { self.1 }
}

fn difference<C: Contains>(container: &C) -> i32 {
    container.last() - container.first()
}

fn main() {
    let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);

    println!("Does container contain {} and {}: {}",
        &number_1, &number_2,
        container.contains(&number_1, &number_2));
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());
    
    println!("The difference is: {}", difference(&container));
}
