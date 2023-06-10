use std::arch::asm;

fn main() {
    let mut a: u64 = 4;
    let mut b: u64 = 4;
    let mut c: u64 = 4;

    // 消すと--releaseビルドでバグる
    println!("a={}", a);
    // println!("b={}", b);
    // println!("c={}", c);

    unsafe {
        asm!(
            "add {0}, {1}",
            "add {0}, {2}",
            inlateout(reg) a,
            in(reg) b,
            in(reg) c,
        );
    }
    // assert_eq!(a, 14);

    println!("a={}", a);
    // println!("b={}", b);
    // println!("c={}", c);
}
