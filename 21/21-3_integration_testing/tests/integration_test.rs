// extern crate we're testing, same as any other code will do.
// 他の外部のコードと同様に、テスト対象のクレートをexternで宣言する。
extern crate adder;

// importing common module.
// 共通のモジュールをインポートする。
mod common;

#[test]
fn test_add() {
    // using common code.
    // 共通のコードを利用する。
    common::setup();
    assert_eq!(adder::add(3, 2), 5);
}