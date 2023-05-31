use std::arch::asm;

fn main() {
    let i: u64 = 3;
    let o: u64;
    unsafe {
        asm!(
            "mov {x}, {y}",
            "add {x}, 5",
            x=out(reg) o,
            y=in(reg) i,
        );
    }
    assert_eq!(o, 8);
    println!("o={}", o);
}
