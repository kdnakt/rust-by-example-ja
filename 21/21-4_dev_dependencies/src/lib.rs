// externing crate for test-only use
// テストにのみ使うクレートをexternで宣言する
#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq; // crate for test-only use. Cannot be used in non-test code.
                                      // テストのためのクレートであり、テスト以外のコードには使えない。

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}