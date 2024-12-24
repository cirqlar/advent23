#[derive(Clone, PartialEq, Eq)]
pub enum Operation {
    And([u8; 3], [u8; 3], [u8; 3]),
    Orr([u8; 3], [u8; 3], [u8; 3]),
    Xor([u8; 3], [u8; 3], [u8; 3]),
}

impl Operation {
    pub fn get_a(&self) -> &[u8] {
        match self {
            Operation::And(a, _, _) => a,
            Operation::Orr(a, _, _) => a,
            Operation::Xor(a, _, _) => a,
        }
    }

    pub fn get_b(&self) -> &[u8] {
        match self {
            Operation::And(_, b, _) => b,
            Operation::Orr(_, b, _) => b,
            Operation::Xor(_, b, _) => b,
        }
    }

    pub fn get_c(&self) -> &[u8; 3] {
        match self {
            Operation::And(_, _, c) => c,
            Operation::Orr(_, _, c) => c,
            Operation::Xor(_, _, c) => c,
        }
    }

    pub fn is_and(&self) -> bool {
        matches!(self, Operation::And(_, _, _))
    }

    pub fn is_orr(&self) -> bool {
        matches!(self, Operation::Orr(_, _, _))
    }

    pub fn is_xor(&self) -> bool {
        matches!(self, Operation::Xor(_, _, _))
    }
}

#[expect(clippy::too_many_arguments)]
pub fn return_c<'a>(
    c: &'a Operation,
    a1: &'a Operation,
    b1: &'a Operation,
    a2: &'a Operation,
    b2: &'a Operation,
    a3: &'a Operation,
    b3: &'a Operation,
    a4: &'a Operation,
    b4: &'a Operation,
) -> &'a [u8; 3] {
    match c {
        x if x == a1 => b1.get_c(),
        x if x == b1 => a1.get_c(),
        x if x == a2 => b2.get_c(),
        x if x == b2 => a2.get_c(),
        x if x == a3 => b3.get_c(),
        x if x == b3 => a3.get_c(),
        x if x == a4 => b4.get_c(),
        x if x == b4 => a4.get_c(),
        x => x.get_c(),
    }
}

pub fn get_key(step: usize, front: u8) -> [u8; 3] {
    match step {
        1 => [front, b'0', b'1'],
        2 => [front, b'0', b'2'],
        3 => [front, b'0', b'3'],
        4 => [front, b'0', b'4'],
        5 => [front, b'0', b'5'],
        6 => [front, b'0', b'6'],
        7 => [front, b'0', b'7'],
        8 => [front, b'0', b'8'],
        9 => [front, b'0', b'9'],

        10 => [front, b'1', b'0'],
        11 => [front, b'1', b'1'],
        12 => [front, b'1', b'2'],
        13 => [front, b'1', b'3'],
        14 => [front, b'1', b'4'],
        15 => [front, b'1', b'5'],
        16 => [front, b'1', b'6'],
        17 => [front, b'1', b'7'],
        18 => [front, b'1', b'8'],
        19 => [front, b'1', b'9'],

        20 => [front, b'2', b'0'],
        21 => [front, b'2', b'1'],
        22 => [front, b'2', b'2'],
        23 => [front, b'2', b'3'],
        24 => [front, b'2', b'4'],
        25 => [front, b'2', b'5'],
        26 => [front, b'2', b'6'],
        27 => [front, b'2', b'7'],
        28 => [front, b'2', b'8'],
        29 => [front, b'2', b'9'],

        30 => [front, b'3', b'0'],
        31 => [front, b'3', b'1'],
        32 => [front, b'3', b'2'],
        33 => [front, b'3', b'3'],
        34 => [front, b'3', b'4'],
        35 => [front, b'3', b'5'],
        36 => [front, b'3', b'6'],
        37 => [front, b'3', b'7'],
        38 => [front, b'3', b'8'],
        39 => [front, b'3', b'9'],

        40 => [front, b'4', b'0'],
        41 => [front, b'4', b'1'],
        42 => [front, b'4', b'2'],
        43 => [front, b'4', b'3'],
        44 => [front, b'4', b'4'],
        45 => [front, b'4', b'5'],
        46 => [front, b'4', b'6'],
        47 => [front, b'4', b'7'],
        48 => [front, b'4', b'8'],
        49 => [front, b'4', b'9'],

        x => unimplemented!("Didn't implement for {}", x),
    }
}
