fn main() {
    // Integer addition
    // 整数の足し算
    println!("1 + 2 = {}", 1u32 + 2);
    println!("1 + 2 = {}", 1u32 + 2i32); // コンパイルエラー！ 4行目は型推論によって 1u32 + 2u32 と同義だということがわかった。

    // Integer subtraction
    // 整数の引き算
    println!("1 - 2 = {}", 1i32 - 2);
    println!("1 - 2 = {}", 1u32 - 2); // コンパイルエラー！ オーバーフローするかどうかをコンパイル時にチェックしてくれるらしい
    println!("1 - 2 = {}", 1u32 - 1); // これは通る。賢い
    // TODO ^ Try changing `1i32` to `1u32` to see why the type is important
    // TODO ^ 型が重要であることを実感するため`1i32`を`1u32`に変更してみましょう。

    // Short-circuiting boolean logic
    // 単純な論理演算子
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operations
    // ビットワイズ演算
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Use underscores to improve readability!
    // 可読性のための`_`（アンダースコア）の使用
    println!("One million is written as {}", 1_000_000u32);
    println!("One million is written as {}", 1_0_0_0_0_0_0u32); // こんなことや
    println!("One million is written as {}", 1___000_000u32);   // こんなこともできる （出力は3行とも1000000）
}
