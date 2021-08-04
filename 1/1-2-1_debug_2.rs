// Derive the `fmt::Debug` implementation for `Structure`. `Structure`
// is a structure which contains a single `i32`.
// `Structure`という構造体のための`fmt::Debug`をderiveしています。
// `Structure`は単一の`i32`をメンバに持っています。
#[derive(Debug)]
struct Structure(i32);

// Put a `Structure` inside of the structure `Deep`. Make it printable
// also.
// `Deep`という構造体の中に`Structure`を入れます。
// また、これをプリント可能にしています。
#[derive(Debug)]
struct Deep(Structure);

fn main() {
    // Printing with `{:?}` is similar to with `{}`.
    // `{:?}`によるプリントは `{}`に似ています。
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor="actor's");

    // `Structure` is printable!
    // `Structure`はプリント可能です！
    println!("Now {:?} will print!", Structure(3));
    
    // The problem with `derive` is there is no control over how
    // the results look. What if I want this to just show a `7`?
    // `derive`を用いることの問題は、結果がどのように見えるか
    // コントロールする方法がないことです。
    // 出力を`7`だけにするためにはどうしたらよいでしょう？
    println!("Now {:?} will print!", Deep(Structure(7)));
}
