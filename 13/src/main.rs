fn used_function() {}

// `#[allow(dead_code)]` is an attribute that disables the `dead_code` lint
// `#[allow(dead_code)]`は`dead_code`リントを抑制するアトリビュートです。
#[allow(dead_code)]
fn unused_function() {}

#[allow(dead_code)]
fn noisy_unused_function() {}
// FIXME ^ Add an attribute to suppress the warning
// FIXME ^ 警告を抑制するアトリビュートを追加しましょう。

fn main() {
    used_function();
}
