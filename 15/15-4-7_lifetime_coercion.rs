// Here, Rust infers a lifetime that is as short as possible.
// The two references are then coerced to that lifetime.
// ここではRustはライフタイムを出来る限り短く見積もり、
// 2つの参照をそのライフタイムに押し込める。
fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
    first * second
}

// `<'a: 'b, 'b>` reads as lifetime `'a` is at least as long as `'b`.
// Here, we take in an `&'a i32` and return a `&'b i32` as a result of coercion.
// `<'a: 'b, 'b>`は「ライフタイム`'a`は最低でも`'b`と同じ長さ」と読める。
// ここでは、`&'a i32`をとり、`&'b i32`に圧縮して返す。
fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
    first
}

fn main() {
    let first = 2; // Longer lifetime
                   // 長いライフタイム
    
    {
        let second = 3; // Shorter lifetime
                        // 短いライフタイム
        
        println!("The product is {}", multiply(&first, &second));
        println!("{} is the first", choose_first(&first, &second));
    };
}
