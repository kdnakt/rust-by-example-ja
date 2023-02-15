// Re-implementation of integer division (/)
// 整数の除法(/)の再実装
fn division(dividend: i32, divisor: i32) -> i32 {
    if divisor == 0 {
        // Division by zero triggers a panic
        // ゼロによる除算はパニックを引き起こす
        panic!("division by zero");
    } else {
        dividend / divisor
    }
}

struct A;

impl Drop for A {
    fn drop(&mut self) {
        println!("A is dropped")
    }
}

// The `main` task
// `main`のタスク
fn main() {
    let _a = A {};

    // This operation will trigger a task failure
    // このオペレーションはタスクの失敗を引き起こす
    division(3, 0);

    println!("This point won't be reached!");

    // `_a`のdropが呼び出される; prints A is dropped!
}

