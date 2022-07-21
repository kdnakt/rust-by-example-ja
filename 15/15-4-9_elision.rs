// `elided_input` and `annotated_input` essentially have identical signatures
// because the lifetime of `elided_input` is inferred by the compiler:
// `elided_input`のライフタイムはコンパイラによって自動的に付与されるため
// 以下の2つは同一のライフタイムシグネチャを持つ。
fn elided_input(x: &i32) {
    println!("`elided_input`: {}", x);
}

fn annotated_input<'a>(x: &'a i32) {
    println!("`annotated_input`: {}", x);
}

// Similarly, `elided_pass` and `annotated_pass` have identical signatures
// because the lifetime is added implicitly to `elided_pass`:
// 同様に、以下の2つの関数も全く同じライフタイムシグネチャを持つ。
static NUM: i32 = 18;

fn elided_pass(x: &i32) -> &i32 { &NUM }

fn annotated_pass<'a>(x: &'a i32) -> &'a i32 { x }

fn main() {
    {
        let x = 3;

        elided_input(&x);
        annotated_input(&x);

        println!("`elided_pass`: {}", elided_pass(&x));
        println!("`annotated_pass`: {}", annotated_pass(&x));
    }
}