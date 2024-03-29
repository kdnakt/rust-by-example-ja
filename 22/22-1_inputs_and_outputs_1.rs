use std::arch::asm;

fn main() {
    let x: u64;
    unsafe {
        // 目的によってレジスタを使い分けないといけない
        asm!("mov {}, 5", out(reg) x);
    }
    assert_eq!(x, 5);
    println!("x={}", x);
}