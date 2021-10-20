// Suppress all warnings from casts which overflow.
// オーバーフローを起こすようなキャスティングによる警告を無視する。
#![allow(overflowing_literals)]

fn main() {
    let decimal = 65.4321_f32;

    // Error! No implicit conversion
    // エラー！ 暗黙的な型変換はできない。
    // let integer: u8 = decimal;
    // FIXME ^ Comment out this line
    // FIXME ^ この行をコメントアウトしましょう。

    // Explicit conversion
    // 明示的な型変換
    let integer = decimal as u8;
    let character = integer as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // when casting any value to an unsigned type, T,
    // std::T::MAX + 1 is added or subtracted until the value
    // fits into the new type
    // 何らかの値を符号なしの型（仮にTとする）へキャスティングすると
    // 値がTに収まるまで、std::T::MAX + 1 が加算あるいは減算される。

    // 1000 already fits in a u16
    // 1000 はすでにu16に収まっているため変化しない。
    println!("1000 is: {} [{:b}]", 1000, 1000);
    println!("1000 as a u16 is: {} [{:b}]", 1000 as u16, 1000 as u16);

    // 1000 - 256 - 256 - 256 = 232
    // Under the hood, the first 8 least significant bits (LSB) are kept,
    // while the rest towards the most significant bit (MSB) get truncated.
    // 水面下では最下位ビットから8bitが使用され、残りの上位ビットが圧縮される形になる。
    println!("1000 as a u8 is : {} [{:b}]", 1000 as u8, 1000 as u8);
    // -1 + 256 = 255
    println!("  -1 as a i8 is : {} [{:b}]", -1i8, -1i8);
    println!("  -1 as a u8 is : {} [{:b}]", (-1i8) as u8, (-1i8) as u8);

    // For positive numbers, this is the same as the modulus
    println!("1000 mod 256 is : {} [{:b}]", 1000 % 256, 1000 % 256);

    // When casting to a signed type, the (bitwise) result is the same as
    // first casting to the corresponding unsigned type. If the most significant
    // bit of that value is 1, then the value is negative.
    // 符号付きの型にキャストする場合、結果は以下の2つを行った場合に等しい
    // 1. 対応する符号なしの型にキャストする。
    // 2. 2の補数(two's complement)をとる

    // Unless it already fits, of course.
    println!(" 128 is: {} [{:b}]", 128, 128);
    println!(" 128 as a i16 is: {} [{:b}]", 128 as i16, 128 as i16);
    
    // 128 as u8 -> 128, whose two's complement in eight bits is:
    // 128をu8にキャストすると128となる。128の8ビットにおける補数は -128
    println!(" 128 as a i8 is : {} [{:b}]", 128 as i8, 128 as i8);

    // repeating the example above
    // 上で示した例から
    // 1000 as u8 -> 232
    println!("1000 as a u8 is : {} [{:b}]", 1000 as u8, 1000 as u8);
    // and the two's complement of 232 is -24
    // が成り立つ。232の8ビットにおける補数は -24
    println!(" 232 as a i8 is : {} [{:b}]", 232 as i8, 232 as i8);
}
