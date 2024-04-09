pub fn u8_to_u32(bits: (u8, u8, u8, u8)) -> u32 {
    let a0 = u32::from(bits.0);
    let a1 = u32::from(bits.1);
    let a2 = u32::from(bits.2);
    let a3 = u32::from(bits.3);

    (a0 << 24) + (a1 << 16) + (a2 << 8) + a3
}

pub fn u32_to_u8(num: u32) -> (u8, u8, u8, u8) {
    let mut num_mut = num;

    let a0 = num_mut & 0xFF;
    num_mut >>= 8;
    let a1 = num_mut & 0xFF;
    num_mut >>= 8;
    let a2 = num_mut & 0xFF;
    num_mut >>= 8;
    let a3 = num_mut;

    (a3 as u8, a2 as u8, a1 as u8, a0 as u8)
}

pub fn i8_to_i32(bits: (i8, i8, i8, i8)) -> i32 {
    u8_to_u32((bits.0 as u8,
               bits.1 as u8,
               bits.2 as u8,
               bits.3 as u8)) as i32
}

pub fn i32_to_i8(num: i32) -> (i8, i8, i8, i8) {
    let result = u32_to_u8(num as u32);
    (result.0 as i8, result.1 as i8, result.2 as i8, result.3 as i8)
}
