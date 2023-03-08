use std::time::Duration;
use std::rc::Rc;
use std::thread;

#[derive(PartialEq, Eq, Debug)]
struct A;

impl Drop for A {
    fn drop(&mut self) {
        println!("A is dropped!")
    }
}

fn main() {
    // この変数宣言で、値をヒープ上に配置する。
    // This variable declaration is where its value is specified.
    // let apple: Arc<&str> = Arc::new("the same apple");
    let apple: Rc<&str> = Rc::new("the same apple");

    for _ in 0..10 {
        // クローンしているのはヒープ上の値への参照であるため、値が実際に複製されることはない。
        // Here there is no value specification as it is a pointer to a reference in the memory heap.
        let apple = Rc::clone(&apple);

        thread::spawn(move || {
            // Arc が使われているので、Arcが所有する値を使ってスレッドをspawnすることができる。
            // 訳注: Rcを使った場合、spawnに渡すラムダ式が変数をキャプチャできずコンパイルエラーが出ます。
            //   E.g. error[E0277]: `Rc<&str>` cannot be sent between threads safely
            //        help: within `[closure@19-9_arc.rs:25:23: 31:10]`, the trait `Send` is not implemented for `Rc<&str>`
            // As Arc was used, threads can be spawned using the value allocated
            // in the Arc variable pointer's location.
            println!("{:?}", apple);
        });
    }

    // Make sure all Arc instances are printed from spawned threads.
    thread::sleep(Duration::from_secs(1));
}
