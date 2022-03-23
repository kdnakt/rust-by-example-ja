// Bind the `deeply::nested::function` path to `other_function`.
// `deeply::nested::function`を`other_function`にバインド
use deeply::nested::function as other_function;

fn function() {
    println!("called `function()`");
}

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called `deeply::nested::function()`");
        }
    }
}

fn main() {
    // Easier access to `deeply::nested::function`
    // `deeply::nested::function`へ、より簡潔にアクセス
    other_function();

    println!("Entering block");
    {
        // This is equivalent to `use deeply::nested::function as function`.
        // This `function()` will shadow the outer one.
        // これは`use deeply::nested::function as function`と同等
        // この`function()`は外の`function()`をシャドウイングする
        use crate::deeply::nested::function;
        function();

        // `use` bindings have a local scope. In this case, the
        // shadowing of `function()` is only in this block.
        // `use`バインディングは局所的なスコープを持つ。
        // この場合には`function()`のシャドウイングはこのブロック内のみ
        function();

        println!("Leaving block");
    }

    function();
}
