// Define a function which takes a generic `F` argument
// bounded by `Fn`, and calls it
// 関数を引数として取り、即座に実行する関数を定義
fn call_me<F: Fn()>(f: F) {
    f();
}


fn call_me_with_args<F: Fn(i32) -> i32>(f: F) -> i32 {
    f(2)
}

// Define a wrapper function satisfying the `Fn` bound
fn function() {
    println!("I'm a function!");
}

fn function_with_args(n: i32) -> i32 {
    println!("I'm a function! {}", n);
    n + 1
}

fn main() {
    // Define a closure satisfying the `Fn` bound
    let closure = || println!("I'm a closure!");

    call_me(closure);
    call_me(function);

    let closure_with_args = |n: i32| {
        println!("I'm a closure! {}", n);
        n + 1
    };

    let c_res = call_me_with_args(closure_with_args);
    println!("c_res {}", c_res);
    let f_res = call_me_with_args(function_with_args);
    println!("f_res {}", f_res);
}