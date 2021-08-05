use std::fmt; // Import `fmt`

// A structure holding two numbers. `Debug` will be derived so the results can
// be contrasted with `Display`.
// 2つの数字を扱うための構造体です。出力を`Display`と比較するため`Debug`
// をDeriveしています
#[derive(Debug)]
struct MinMax(i64, i64);

// Implement `Display` for `MinMax`.
// `MinMax`用の`Display`を実装しています。
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "({}, {})", self.0, self.1)
    }
}

// Define a structure where the fields are nameable for comparison.
// 比較のため、フィールドに名前をつけれる様な構造体を定義しましょう
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}


// Similarly, implement `Display` for `Point2D`
// 先程と同様にして、Point2D用の`Display`を実装しています。
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        // `x`と`y`のみが明示的になるようにカスタマイズ
        write!(f, "x: {}, y: {}", self.x, self.y)   // 変数名を定義していると `self.0, self.1` のように順序での変数指定はできない
    }
}

// 出力内容がどうであれ、Binary を実装していれば {:b} を出力することはできる
impl fmt::Binary for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        // `x`と`y`のみが明示的になるようにカスタマイズ
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

// 演習
// 上記の例のアウトプットを確認し、Point2D構造体を参考として、
// 複素数を格納するための構造体を定義しましょう。
// うまく行けば以下のように出力されるはずです。
// ```
// Display: 3.3 + 7.2i
// Debug: Complex { real: 3.3, imag: 7.2 }
// ```

#[derive(Debug)]
struct Complex{
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}



fn main() {
    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range =   MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small is {small}",
             small = small_range,
             big = big_range);

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    // Error. Both `Debug` and `Display` were implemented, but `{:b}`
    // requires `fmt::Binary` to be implemented. This will not work.
    // `Debug`と`Display`は実装されていますが、`fmt::Binary`はされていないため
    // `{:b}`使用している以下の例はエラーになります、
    println!("What does Point2D look like in binary: {:b}?", point);

    // 演習の動作確認
    let complex = Complex { real: 3.3, imag: 7.2 };

    println!("Complex:");
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);
    println!("Debug: {:#?}", complex);  // 「見栄えの良いプリント」

}
