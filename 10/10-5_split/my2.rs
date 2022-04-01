// Similarly `mod inaccessible` and `mod nested` will locate the `nested.rs`
// and `inaccessible.rs` files and insert them here under their respective
// modules
// 同様に`mod inaccessible`、`mod nested`によって、`nested.rs`、`inaccessible.rs`の内容をこの中で使用することができるようになる。
// 訳注: `pub`をつけないかぎり、この中でしか使用できない。
mod inaccessible;
pub mod nested;

pub fn function() {
    println!("called `my2::function()`");
}

fn private_function() {
    println!("called `my2::private_function()`");
}

pub fn indirect_access() {
    print!("called `my2::indirect_access()`, that\n> ");

    private_function();

    inaccessible::public_function();
}
