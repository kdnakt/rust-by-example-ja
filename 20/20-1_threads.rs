use std::thread;

const NTHREADS: u32 = 10;

// This is the `main` thread
// この関数は`main`スレッドで実行される。
fn main() {
    // Make a vector to hold the children which are spawned.
    // spawnされるクロージャを保持するためのベクタ
    //let mut children = vec![];

    thread::scope(|s| {
        for i in 0..NTHREADS {
            // Spin up another thread
            // 新しいスレッドを起動
            s.spawn(move || {
                println!("this is thread number {}", i);
            });
        }    
    })

    // for child in children {
    //     // Wait for the thread to finish. Returns a result.
    //     // 子スレッドが終了するのを待ち、結果を返す。
    //     let _ = child.join();
    // }
}