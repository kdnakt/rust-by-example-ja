use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
static NTHREADS: i32 = 3;
fn main() {
    // Channels have two endpoints: the `Sender<T>` and the `Receiver<T>`,
    // where `T` is the type of the message to be transferred
    // (type annotation is superfluous)
    // チャネルには`Sender<T>`と`Receiver<T>`という2つのエンドポイントがある。
    // ここで、`T`は送信されるメッセージの型である。
    // （型アノテーションは必須ではない。）
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
    let mut children = Vec::new();
    for id in 0..NTHREADS {
        // The sender endpoint can be copied
        // 送信者エンドポイントはコピーすることができる。
        let thread_tx = tx.clone();
        // Each thread will send its id via the channel
        // ここでは、それぞれのスレッドが自身のIDを送信している。
        let child = thread::spawn(move || {
            // The thread takes ownership over `thread_tx`
            // Each thread queues a message in the channel
            // スレッドは`thread_tx`の所有権をとり、それぞれのスレッドは
            // メッセージをチャネルにキューイングする。
            // unwrapするなどしてエラーハンドリングしましょう。
            thread_tx.send(id).unwrap();
            // thread_tx.send(id);
            // Sending is a non-blocking operation, the thread will continue
            // immediately after sending its message
            // 送信はノンブロッキングなオペレーションなので、
            // メッセージを送信した後もすぐに実行を継続する。
            println!("thread {} finished", id);
        });
        children.push(child);
    }
    // Here, all the messages are collected
    // ここで、全てのメッセージが収集される。
    let mut ids = Vec::with_capacity(NTHREADS as usize);
    for _ in 0..NTHREADS {
        // The `recv` method picks a message from the channel
        // `recv` will block the current thread if there are no messages available
        // `recv`メソッドはチャネルからメッセージを取り出す。
        // もし取り出せるメッセージが存在しない場合、`recv`は
        // 現在のスレッドをブロックする。
        ids.push(rx.recv());
    }
    // Wait for the threads to complete any remaining work
    // join()が実行されないと、threadの中の処理が終わったか保証できない
    for child in children {
        child.join().expect("oops! the child thread panicked");
    }
    // Show the order in which the messages were sent
    // メッセージが送信された順番を表示
    println!("{:?}", ids);
    drop(rx);
    // tx.send(4).unwrap();
    tx.send(4);
}