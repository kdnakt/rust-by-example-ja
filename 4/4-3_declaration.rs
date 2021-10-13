fn main() {
    // 変数を宣言
    let a_binding;
    
    // Error! Use of uninitialized binding
    // エラー！ 初期化していない変数の使用
    // println!("a_binding: {}", a_binding);
    // FIXME ^ Comment out this line
    // FIXME ^ この行をコメントアウトしましょう。

    {
        let x = 2;
        // Initialize the binding
        // 変数を初期化
        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);

    let _a = A; // _aはスコープを外れた後にdrop()される
    let _b: A;  // _bは初期化されていないのでdrop()されない
}   // prints "Drop A"

struct A;

impl Drop for A {
    fn drop(&mut self) {
        println!("Drop A");
    }
}
