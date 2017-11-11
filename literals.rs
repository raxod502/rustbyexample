#![allow(unused_variables)]

fn main() {
    println!("1 + 2 = {}", 1 + 2);
    println!("1 - 2 = {}", 1 - 2);
    // println!("1 - 2 = {}", 1u32 - 2);

    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    println!("0011 AND 0101 is {:04b}", 0b011u32 & 0b0101u32);

    println!("With signed: {:04b}", 0b011 & 0b0101);
    println!("Negative binary number: {}", -0b111);
    println!("In binary: {:b}", -0b111);

    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101u32);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101u32);

    println!("1 << 5 is {}", 0b1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    println!("One million is written as {}", 1_000_000);

    let unsigned = 100u32;
    let signed = unsigned;

    let small_unsigned = 250u8;
    // let small_signed: i8 = small_unsigned;

    // let bad_sum: i32 = 50u32 + 5;
}
