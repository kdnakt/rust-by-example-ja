// `NanoSecond` is a new name for `u64`.
// `NanoSecond` を `u64`の別名として使用する。
type NanoSecond = u64;
type Inch = u64;

// Use an attribute to silence warning.
// 警告を抑えるアトリビュートを使用。
#[allow(non_camel_case_types)]
type u64_t = u64;
// TODO ^ Try removing the attribute
// TODO ^ アトリビュートを使用しない場合、どうなるか見てみましょう。

#[allow(dead_code)]
type InchResult<T> = Result<T, Inch>;

fn main() {
    // `NanoSecond` = `Inch` = `u64_t` = `u64`.
    let nanoseconds: NanoSecond = 5 as u64_t;
    let inches: Inch = 2 as u64_t;

    // Note that type aliases *don't* provide any extra type safety, because
    // aliases are *not* new types
    // 型のエイリアスは、元の型をより型安全にしてくれる **わけではない** ことに注意しましょう。
    // なぜならば、エイリアスは新たな型を定義している **わけではない** からです。
    println!("{} nanoseconds + {} inches = {} unit?",
             nanoseconds,
             inches,
             nanoseconds + inches);
}
