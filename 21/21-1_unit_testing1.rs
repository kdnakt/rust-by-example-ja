pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// This is a really bad adding function, its purpose is to fail in this
// example.
// 誤った加算をする関数がテストに通らないことを示す。
#[allow(dead_code)]
fn bad_add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    // 外部のスコープから（mod testsに）名前をインポートする便利なイディオム。
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_bad_add() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        // このアサーションはパニックして、テストは失敗する。
        // プライベートな関数もテストすることができる。
        assert_eq!(bad_add(1, 2), 3);
    }
}
