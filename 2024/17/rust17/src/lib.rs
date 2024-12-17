pub mod part1;
pub mod part2;

#[cfg(target_os = "windows")]
pub const NEWLINE_OFFSET: usize = 2;
#[cfg(not(target_os = "windows"))]
pub const NEWLINE_OFFSET: usize = 1;

pub fn get_reg_operand_a(operand: &u8) -> usize {
    (*operand - b'0').into()
}

pub fn get_reg_operand_b(operand: u8) -> usize {
    match operand {
        b'0' => 0,
        b'1' => 1,
        b'2' => 2,
        b'3' => 3,
        b'4' => 4,
        b'5' => 5,
        b'6' => 6,
        b'7' => 7,
        b'8' => 8,
        b'9' => 9,
        x => panic!("Invalid reg operand {:?}", x),
    }
}
