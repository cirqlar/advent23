// +---+---+---+
// | 7 | 8 | 9 |
// +---+---+---+
// | 4 | 5 | 6 |
// +---+---+---+
// | 1 | 2 | 3 |
// +---+---+---+
//     | 0 | A |
//     +---+---+
#[rustfmt::skip]
pub const NUMPAD_MAP: [&[u8]; 121] = [
    /* 0 */ b"A",      b"^<A",   b"^A",    b"^>A",    b"^^<A",    b"^^A",    b"^^>A",    b"^^^<A",    b"^^^A",    b"^^^>A",   b">A",
    /* 1 */ b">vA",    b"A",     b">A",    b">>A",    b"^A",      b"^>A",    b"^>>A",    b"^^A",      b"^^>A",    b"^^>>A",   b">>vA",
    /* 2 */ b"vA",     b"<A",    b"A",     b">A",     b"^<A",     b"^A",     b"^>A",     b"^^<A",     b"^^A",     b"^^>A",    b">vA",
    /* 3 */ b"v<A",    b"<<A",   b"<A",    b"A",      b"^<<A",    b"^<A",    b"^A",      b"^^<<A",    b"^^<A",    b"^^A",     b"vA",
    /* 4 */ b">vvA",   b"vA",    b">vA",   b">>vA",   b"A",       b">A",     b">>A",     b"^A",       b"^>A",     b"^>>A",    b">>vvA",
    /* 5 */ b"vvA",    b"v<A",   b"vA",    b">vA",    b"<A",      b"A",      b">A",      b"^<A",      b"^A",      b"^>A",     b">vvA",
    /* 6 */ b"vv<A",   b"v<<A",  b"v<A",   b"vA",     b"<<A",     b"<A",     b"A",       b"^<<A",     b"^<A",     b"^A",      b"vvA",
    /* 7 */ b">vvvA",  b"vvA",   b"vv>A",  b">>vvA",  b"vA",      b">vA",    b">>vA",    b"A",        b">A",      b">>A",     b">>vvvA",
    /* 8 */ b"vvvA",   b"vv<A",  b"vvA",   b"vv>A",   b"v<A",     b"vA",     b">vA",     b"<A",       b"A",       b">A",      b">vvvA",
    /* 9 */ b"vvv<A",  b"vv<<A", b"vv<A",  b"vvA",    b"v<<A",    b"v<A",    b"vA",      b"<<A",      b"<A",      b"A",       b"vvvA",
    /* A */ b"<A",     b"^<<A",  b"^<A",   b"^A",     b"^^<<A",   b"^^<A",   b"^^A",     b"^^^<<A",   b"^^^<A",   b"^^^A",    b"A",
];

pub fn num_to_num(ch: u8) -> usize {
    match ch {
        x if x.is_ascii_digit() => (x - b'0').into(),
        b'A' => 10,
        x => panic!("We shouldn't do that for {}", x as char),
    }
}

//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+
#[rustfmt::skip]
pub const ARROW_MAP: [&[u8]; 25] = [
    /* ^ */ b"A",   b"vA",   b">vA",   b"v<A",   b">A",
    /* v */ b"^A",  b"A",    b">A",    b"<A",    b"^>A",
    /* > */ b"^<A", b"<A",   b"A",     b"<<A",   b"^A",
    /* < */ b">^A", b">A",   b">>A",   b"A",     b">>^A",
    /* A */ b"<A",  b"v<A",  b"vA",    b"v<<A",  b"A",
];

pub fn arrow_to_num(ch: u8) -> usize {
    match ch {
        b'^' => 0,
        b'v' => 1,
        b'>' => 2,
        b'<' => 3,
        b'A' => 4,
        x => panic!("We shouldn't do that for {}", x as char),
    }
}
