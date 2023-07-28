use std::arch::asm;

fn main() {
    let mut x: u16 = 0xab;
    let ax: [u8; 2] = unsafe { std::mem::transmute::<u16, [u8; 2]>(x) };
    println!("{:?}", &ax);
    unsafe {
        asm!("mov {0:h}, {0:l}", inout(reg_abcd) x);
        // asm!("mov {0:l}, {0:h}", inout(reg_abcd) x);
        // asm!("mov ah, al", inout("ax") x);
    }
    let ax: [u8; 2] = unsafe { std::mem::transmute::<u16, [u8; 2]>(x) };
    println!("{:?}", &ax);
    assert_eq!(x, 0xabab);
}