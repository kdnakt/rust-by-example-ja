// This declaration will look for a file named `my.rs` or `my/mod.rs` and will
// insert its contents inside a module named `my` under this scope
// このように宣言すると、`my.rs`または、`my/mod.rs`という名のファイルを探し、
// その内容をこのファイル中で`my`という名から使用することができるようにします。
mod my;
mod my2;

fn function() {
    println!("called `function()`");
}

fn main() {
    // call function here
    function();

    // call function via mod.rs
    my::function();
    my::indirect_access();
    my::nested::function();
    nested::function();
    
    // compilation error
    // my::nested::private_function();
    // my::inaccessible::public_function();

    // call function via my2.rs
    my2::function();
    my2::indirect_access();
    my2::nested::function();
}