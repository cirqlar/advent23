type TwelveTuple<'a> = (
    &'a usize,
    &'a usize,
    &'a usize,
    &'a usize,
    &'a usize,
    &'a usize,
    &'a usize,
    &'a usize,
    &'a usize,
    &'a usize,
    &'a usize,
    &'a usize,
);

pub fn twelve(tuple: TwelveTuple, connections: &[Option<Vec<usize>>]) -> bool {
    connections[*tuple.0].as_ref().unwrap().contains(tuple.1)
        && connections[*tuple.0].as_ref().unwrap().contains(tuple.2)
        && connections[*tuple.0].as_ref().unwrap().contains(tuple.3)
        && connections[*tuple.0].as_ref().unwrap().contains(tuple.4)
        && connections[*tuple.0].as_ref().unwrap().contains(tuple.5)
        && connections[*tuple.0].as_ref().unwrap().contains(tuple.6)
        && connections[*tuple.0].as_ref().unwrap().contains(tuple.7)
        && connections[*tuple.0].as_ref().unwrap().contains(tuple.8)
        && connections[*tuple.0].as_ref().unwrap().contains(tuple.9)
        && connections[*tuple.0].as_ref().unwrap().contains(tuple.10)
        && connections[*tuple.0].as_ref().unwrap().contains(tuple.11)
        && connections[*tuple.1].as_ref().unwrap().contains(tuple.2)
        && connections[*tuple.1].as_ref().unwrap().contains(tuple.3)
        && connections[*tuple.1].as_ref().unwrap().contains(tuple.4)
        && connections[*tuple.1].as_ref().unwrap().contains(tuple.5)
        && connections[*tuple.1].as_ref().unwrap().contains(tuple.6)
        && connections[*tuple.1].as_ref().unwrap().contains(tuple.7)
        && connections[*tuple.1].as_ref().unwrap().contains(tuple.8)
        && connections[*tuple.1].as_ref().unwrap().contains(tuple.9)
        && connections[*tuple.1].as_ref().unwrap().contains(tuple.10)
        && connections[*tuple.1].as_ref().unwrap().contains(tuple.11)
        && connections[*tuple.2].as_ref().unwrap().contains(tuple.3)
        && connections[*tuple.2].as_ref().unwrap().contains(tuple.4)
        && connections[*tuple.2].as_ref().unwrap().contains(tuple.5)
        && connections[*tuple.2].as_ref().unwrap().contains(tuple.6)
        && connections[*tuple.2].as_ref().unwrap().contains(tuple.7)
        && connections[*tuple.2].as_ref().unwrap().contains(tuple.8)
        && connections[*tuple.2].as_ref().unwrap().contains(tuple.9)
        && connections[*tuple.2].as_ref().unwrap().contains(tuple.10)
        && connections[*tuple.2].as_ref().unwrap().contains(tuple.11)
        && connections[*tuple.3].as_ref().unwrap().contains(tuple.4)
        && connections[*tuple.3].as_ref().unwrap().contains(tuple.5)
        && connections[*tuple.3].as_ref().unwrap().contains(tuple.6)
        && connections[*tuple.3].as_ref().unwrap().contains(tuple.7)
        && connections[*tuple.3].as_ref().unwrap().contains(tuple.8)
        && connections[*tuple.3].as_ref().unwrap().contains(tuple.9)
        && connections[*tuple.3].as_ref().unwrap().contains(tuple.10)
        && connections[*tuple.3].as_ref().unwrap().contains(tuple.11)
        && connections[*tuple.4].as_ref().unwrap().contains(tuple.5)
        && connections[*tuple.4].as_ref().unwrap().contains(tuple.6)
        && connections[*tuple.4].as_ref().unwrap().contains(tuple.7)
        && connections[*tuple.4].as_ref().unwrap().contains(tuple.8)
        && connections[*tuple.4].as_ref().unwrap().contains(tuple.9)
        && connections[*tuple.4].as_ref().unwrap().contains(tuple.10)
        && connections[*tuple.4].as_ref().unwrap().contains(tuple.11)
        && connections[*tuple.5].as_ref().unwrap().contains(tuple.6)
        && connections[*tuple.5].as_ref().unwrap().contains(tuple.7)
        && connections[*tuple.5].as_ref().unwrap().contains(tuple.8)
        && connections[*tuple.5].as_ref().unwrap().contains(tuple.9)
        && connections[*tuple.5].as_ref().unwrap().contains(tuple.10)
        && connections[*tuple.5].as_ref().unwrap().contains(tuple.11)
        && connections[*tuple.6].as_ref().unwrap().contains(tuple.7)
        && connections[*tuple.6].as_ref().unwrap().contains(tuple.8)
        && connections[*tuple.6].as_ref().unwrap().contains(tuple.9)
        && connections[*tuple.6].as_ref().unwrap().contains(tuple.10)
        && connections[*tuple.6].as_ref().unwrap().contains(tuple.11)
        && connections[*tuple.7].as_ref().unwrap().contains(tuple.8)
        && connections[*tuple.7].as_ref().unwrap().contains(tuple.9)
        && connections[*tuple.7].as_ref().unwrap().contains(tuple.10)
        && connections[*tuple.7].as_ref().unwrap().contains(tuple.11)
        && connections[*tuple.8].as_ref().unwrap().contains(tuple.9)
        && connections[*tuple.8].as_ref().unwrap().contains(tuple.10)
        && connections[*tuple.8].as_ref().unwrap().contains(tuple.11)
        && connections[*tuple.9].as_ref().unwrap().contains(tuple.10)
        && connections[*tuple.9].as_ref().unwrap().contains(tuple.11)
        && connections[*tuple.10].as_ref().unwrap().contains(tuple.11)
}

type ElevenTuple<'a> = (
    &'a usize,
    &'a usize,
    &'a usize,
    &'a usize,
    &'a usize,
    &'a usize,
    &'a usize,
    &'a usize,
    &'a usize,
    &'a usize,
    &'a usize,
);

pub fn eleven(tuple: ElevenTuple, connections: &[Option<Vec<usize>>]) -> bool {
    connections[*tuple.0].as_ref().unwrap().contains(tuple.1)
        && connections[*tuple.0].as_ref().unwrap().contains(tuple.2)
        && connections[*tuple.0].as_ref().unwrap().contains(tuple.3)
        && connections[*tuple.0].as_ref().unwrap().contains(tuple.4)
        && connections[*tuple.0].as_ref().unwrap().contains(tuple.5)
        && connections[*tuple.0].as_ref().unwrap().contains(tuple.6)
        && connections[*tuple.0].as_ref().unwrap().contains(tuple.7)
        && connections[*tuple.0].as_ref().unwrap().contains(tuple.8)
        && connections[*tuple.0].as_ref().unwrap().contains(tuple.9)
        && connections[*tuple.0].as_ref().unwrap().contains(tuple.10)
        && connections[*tuple.1].as_ref().unwrap().contains(tuple.2)
        && connections[*tuple.1].as_ref().unwrap().contains(tuple.3)
        && connections[*tuple.1].as_ref().unwrap().contains(tuple.4)
        && connections[*tuple.1].as_ref().unwrap().contains(tuple.5)
        && connections[*tuple.1].as_ref().unwrap().contains(tuple.6)
        && connections[*tuple.1].as_ref().unwrap().contains(tuple.7)
        && connections[*tuple.1].as_ref().unwrap().contains(tuple.8)
        && connections[*tuple.1].as_ref().unwrap().contains(tuple.9)
        && connections[*tuple.1].as_ref().unwrap().contains(tuple.10)
        && connections[*tuple.2].as_ref().unwrap().contains(tuple.3)
        && connections[*tuple.2].as_ref().unwrap().contains(tuple.4)
        && connections[*tuple.2].as_ref().unwrap().contains(tuple.5)
        && connections[*tuple.2].as_ref().unwrap().contains(tuple.6)
        && connections[*tuple.2].as_ref().unwrap().contains(tuple.7)
        && connections[*tuple.2].as_ref().unwrap().contains(tuple.8)
        && connections[*tuple.2].as_ref().unwrap().contains(tuple.9)
        && connections[*tuple.2].as_ref().unwrap().contains(tuple.10)
        && connections[*tuple.3].as_ref().unwrap().contains(tuple.4)
        && connections[*tuple.3].as_ref().unwrap().contains(tuple.5)
        && connections[*tuple.3].as_ref().unwrap().contains(tuple.6)
        && connections[*tuple.3].as_ref().unwrap().contains(tuple.7)
        && connections[*tuple.3].as_ref().unwrap().contains(tuple.8)
        && connections[*tuple.3].as_ref().unwrap().contains(tuple.9)
        && connections[*tuple.3].as_ref().unwrap().contains(tuple.10)
        && connections[*tuple.4].as_ref().unwrap().contains(tuple.5)
        && connections[*tuple.4].as_ref().unwrap().contains(tuple.6)
        && connections[*tuple.4].as_ref().unwrap().contains(tuple.7)
        && connections[*tuple.4].as_ref().unwrap().contains(tuple.8)
        && connections[*tuple.4].as_ref().unwrap().contains(tuple.9)
        && connections[*tuple.4].as_ref().unwrap().contains(tuple.10)
        && connections[*tuple.5].as_ref().unwrap().contains(tuple.6)
        && connections[*tuple.5].as_ref().unwrap().contains(tuple.7)
        && connections[*tuple.5].as_ref().unwrap().contains(tuple.8)
        && connections[*tuple.5].as_ref().unwrap().contains(tuple.9)
        && connections[*tuple.5].as_ref().unwrap().contains(tuple.10)
        && connections[*tuple.6].as_ref().unwrap().contains(tuple.7)
        && connections[*tuple.6].as_ref().unwrap().contains(tuple.8)
        && connections[*tuple.6].as_ref().unwrap().contains(tuple.9)
        && connections[*tuple.6].as_ref().unwrap().contains(tuple.10)
        && connections[*tuple.7].as_ref().unwrap().contains(tuple.8)
        && connections[*tuple.7].as_ref().unwrap().contains(tuple.9)
        && connections[*tuple.7].as_ref().unwrap().contains(tuple.10)
        && connections[*tuple.8].as_ref().unwrap().contains(tuple.9)
        && connections[*tuple.8].as_ref().unwrap().contains(tuple.10)
        && connections[*tuple.9].as_ref().unwrap().contains(tuple.10)
}

type TenTuple<'a> = (
    &'a usize,
    &'a usize,
    &'a usize,
    &'a usize,
    &'a usize,
    &'a usize,
    &'a usize,
    &'a usize,
    &'a usize,
    &'a usize,
);

pub fn ten(tuple: TenTuple, connections: &[Option<Vec<usize>>]) -> bool {
    connections[*tuple.0].as_ref().unwrap().contains(tuple.1)
        && connections[*tuple.0].as_ref().unwrap().contains(tuple.2)
        && connections[*tuple.0].as_ref().unwrap().contains(tuple.3)
        && connections[*tuple.0].as_ref().unwrap().contains(tuple.4)
        && connections[*tuple.0].as_ref().unwrap().contains(tuple.5)
        && connections[*tuple.0].as_ref().unwrap().contains(tuple.6)
        && connections[*tuple.0].as_ref().unwrap().contains(tuple.7)
        && connections[*tuple.0].as_ref().unwrap().contains(tuple.8)
        && connections[*tuple.0].as_ref().unwrap().contains(tuple.9)
        && connections[*tuple.1].as_ref().unwrap().contains(tuple.2)
        && connections[*tuple.1].as_ref().unwrap().contains(tuple.3)
        && connections[*tuple.1].as_ref().unwrap().contains(tuple.4)
        && connections[*tuple.1].as_ref().unwrap().contains(tuple.5)
        && connections[*tuple.1].as_ref().unwrap().contains(tuple.6)
        && connections[*tuple.1].as_ref().unwrap().contains(tuple.7)
        && connections[*tuple.1].as_ref().unwrap().contains(tuple.8)
        && connections[*tuple.1].as_ref().unwrap().contains(tuple.9)
        && connections[*tuple.2].as_ref().unwrap().contains(tuple.3)
        && connections[*tuple.2].as_ref().unwrap().contains(tuple.4)
        && connections[*tuple.2].as_ref().unwrap().contains(tuple.5)
        && connections[*tuple.2].as_ref().unwrap().contains(tuple.6)
        && connections[*tuple.2].as_ref().unwrap().contains(tuple.7)
        && connections[*tuple.2].as_ref().unwrap().contains(tuple.8)
        && connections[*tuple.2].as_ref().unwrap().contains(tuple.9)
        && connections[*tuple.3].as_ref().unwrap().contains(tuple.4)
        && connections[*tuple.3].as_ref().unwrap().contains(tuple.5)
        && connections[*tuple.3].as_ref().unwrap().contains(tuple.6)
        && connections[*tuple.3].as_ref().unwrap().contains(tuple.7)
        && connections[*tuple.3].as_ref().unwrap().contains(tuple.8)
        && connections[*tuple.3].as_ref().unwrap().contains(tuple.9)
        && connections[*tuple.4].as_ref().unwrap().contains(tuple.5)
        && connections[*tuple.4].as_ref().unwrap().contains(tuple.6)
        && connections[*tuple.4].as_ref().unwrap().contains(tuple.7)
        && connections[*tuple.4].as_ref().unwrap().contains(tuple.8)
        && connections[*tuple.4].as_ref().unwrap().contains(tuple.9)
        && connections[*tuple.5].as_ref().unwrap().contains(tuple.6)
        && connections[*tuple.5].as_ref().unwrap().contains(tuple.7)
        && connections[*tuple.5].as_ref().unwrap().contains(tuple.8)
        && connections[*tuple.5].as_ref().unwrap().contains(tuple.9)
        && connections[*tuple.6].as_ref().unwrap().contains(tuple.7)
        && connections[*tuple.6].as_ref().unwrap().contains(tuple.8)
        && connections[*tuple.6].as_ref().unwrap().contains(tuple.9)
        && connections[*tuple.7].as_ref().unwrap().contains(tuple.8)
        && connections[*tuple.7].as_ref().unwrap().contains(tuple.9)
        && connections[*tuple.8].as_ref().unwrap().contains(tuple.9)
}

type NineTuple<'a> = (
    &'a usize,
    &'a usize,
    &'a usize,
    &'a usize,
    &'a usize,
    &'a usize,
    &'a usize,
    &'a usize,
    &'a usize,
);

pub fn nine(tuple: NineTuple, connections: &[Option<Vec<usize>>]) -> bool {
    connections[*tuple.0].as_ref().unwrap().contains(tuple.1)
        && connections[*tuple.0].as_ref().unwrap().contains(tuple.2)
        && connections[*tuple.0].as_ref().unwrap().contains(tuple.3)
        && connections[*tuple.0].as_ref().unwrap().contains(tuple.4)
        && connections[*tuple.0].as_ref().unwrap().contains(tuple.5)
        && connections[*tuple.0].as_ref().unwrap().contains(tuple.6)
        && connections[*tuple.0].as_ref().unwrap().contains(tuple.7)
        && connections[*tuple.0].as_ref().unwrap().contains(tuple.8)
        && connections[*tuple.1].as_ref().unwrap().contains(tuple.2)
        && connections[*tuple.1].as_ref().unwrap().contains(tuple.3)
        && connections[*tuple.1].as_ref().unwrap().contains(tuple.4)
        && connections[*tuple.1].as_ref().unwrap().contains(tuple.5)
        && connections[*tuple.1].as_ref().unwrap().contains(tuple.6)
        && connections[*tuple.1].as_ref().unwrap().contains(tuple.7)
        && connections[*tuple.1].as_ref().unwrap().contains(tuple.8)
        && connections[*tuple.2].as_ref().unwrap().contains(tuple.3)
        && connections[*tuple.2].as_ref().unwrap().contains(tuple.4)
        && connections[*tuple.2].as_ref().unwrap().contains(tuple.5)
        && connections[*tuple.2].as_ref().unwrap().contains(tuple.6)
        && connections[*tuple.2].as_ref().unwrap().contains(tuple.7)
        && connections[*tuple.2].as_ref().unwrap().contains(tuple.8)
        && connections[*tuple.3].as_ref().unwrap().contains(tuple.4)
        && connections[*tuple.3].as_ref().unwrap().contains(tuple.5)
        && connections[*tuple.3].as_ref().unwrap().contains(tuple.6)
        && connections[*tuple.3].as_ref().unwrap().contains(tuple.7)
        && connections[*tuple.3].as_ref().unwrap().contains(tuple.8)
        && connections[*tuple.4].as_ref().unwrap().contains(tuple.5)
        && connections[*tuple.4].as_ref().unwrap().contains(tuple.6)
        && connections[*tuple.4].as_ref().unwrap().contains(tuple.7)
        && connections[*tuple.4].as_ref().unwrap().contains(tuple.8)
        && connections[*tuple.5].as_ref().unwrap().contains(tuple.6)
        && connections[*tuple.5].as_ref().unwrap().contains(tuple.7)
        && connections[*tuple.5].as_ref().unwrap().contains(tuple.8)
        && connections[*tuple.6].as_ref().unwrap().contains(tuple.7)
        && connections[*tuple.6].as_ref().unwrap().contains(tuple.8)
        && connections[*tuple.7].as_ref().unwrap().contains(tuple.8)
}

type EightTuple<'a> = (
    &'a usize,
    &'a usize,
    &'a usize,
    &'a usize,
    &'a usize,
    &'a usize,
    &'a usize,
    &'a usize,
);

pub fn eight(tuple: EightTuple, connections: &[Option<Vec<usize>>]) -> bool {
    connections[*tuple.0].as_ref().unwrap().contains(tuple.1)
        && connections[*tuple.0].as_ref().unwrap().contains(tuple.2)
        && connections[*tuple.0].as_ref().unwrap().contains(tuple.3)
        && connections[*tuple.0].as_ref().unwrap().contains(tuple.4)
        && connections[*tuple.0].as_ref().unwrap().contains(tuple.5)
        && connections[*tuple.0].as_ref().unwrap().contains(tuple.6)
        && connections[*tuple.0].as_ref().unwrap().contains(tuple.7)
        && connections[*tuple.1].as_ref().unwrap().contains(tuple.2)
        && connections[*tuple.1].as_ref().unwrap().contains(tuple.3)
        && connections[*tuple.1].as_ref().unwrap().contains(tuple.4)
        && connections[*tuple.1].as_ref().unwrap().contains(tuple.5)
        && connections[*tuple.1].as_ref().unwrap().contains(tuple.6)
        && connections[*tuple.1].as_ref().unwrap().contains(tuple.7)
        && connections[*tuple.2].as_ref().unwrap().contains(tuple.3)
        && connections[*tuple.2].as_ref().unwrap().contains(tuple.4)
        && connections[*tuple.2].as_ref().unwrap().contains(tuple.5)
        && connections[*tuple.2].as_ref().unwrap().contains(tuple.6)
        && connections[*tuple.2].as_ref().unwrap().contains(tuple.7)
        && connections[*tuple.3].as_ref().unwrap().contains(tuple.4)
        && connections[*tuple.3].as_ref().unwrap().contains(tuple.5)
        && connections[*tuple.3].as_ref().unwrap().contains(tuple.6)
        && connections[*tuple.3].as_ref().unwrap().contains(tuple.7)
        && connections[*tuple.4].as_ref().unwrap().contains(tuple.5)
        && connections[*tuple.4].as_ref().unwrap().contains(tuple.6)
        && connections[*tuple.4].as_ref().unwrap().contains(tuple.7)
        && connections[*tuple.5].as_ref().unwrap().contains(tuple.6)
        && connections[*tuple.5].as_ref().unwrap().contains(tuple.7)
        && connections[*tuple.6].as_ref().unwrap().contains(tuple.7)
}

type SevenTuple<'a> = (
    &'a usize,
    &'a usize,
    &'a usize,
    &'a usize,
    &'a usize,
    &'a usize,
    &'a usize,
);

pub fn seven(tuple: SevenTuple, connections: &[Option<Vec<usize>>]) -> bool {
    connections[*tuple.0].as_ref().unwrap().contains(tuple.1)
        && connections[*tuple.0].as_ref().unwrap().contains(tuple.2)
        && connections[*tuple.0].as_ref().unwrap().contains(tuple.3)
        && connections[*tuple.0].as_ref().unwrap().contains(tuple.4)
        && connections[*tuple.0].as_ref().unwrap().contains(tuple.5)
        && connections[*tuple.0].as_ref().unwrap().contains(tuple.6)
        && connections[*tuple.1].as_ref().unwrap().contains(tuple.2)
        && connections[*tuple.1].as_ref().unwrap().contains(tuple.3)
        && connections[*tuple.1].as_ref().unwrap().contains(tuple.4)
        && connections[*tuple.1].as_ref().unwrap().contains(tuple.5)
        && connections[*tuple.1].as_ref().unwrap().contains(tuple.6)
        && connections[*tuple.2].as_ref().unwrap().contains(tuple.3)
        && connections[*tuple.2].as_ref().unwrap().contains(tuple.4)
        && connections[*tuple.2].as_ref().unwrap().contains(tuple.5)
        && connections[*tuple.2].as_ref().unwrap().contains(tuple.6)
        && connections[*tuple.3].as_ref().unwrap().contains(tuple.4)
        && connections[*tuple.3].as_ref().unwrap().contains(tuple.5)
        && connections[*tuple.3].as_ref().unwrap().contains(tuple.6)
        && connections[*tuple.4].as_ref().unwrap().contains(tuple.5)
        && connections[*tuple.4].as_ref().unwrap().contains(tuple.6)
        && connections[*tuple.5].as_ref().unwrap().contains(tuple.6)
}

type SixTuple<'a> = (
    &'a usize,
    &'a usize,
    &'a usize,
    &'a usize,
    &'a usize,
    &'a usize,
);

pub fn six(tuple: SixTuple, connections: &[Option<Vec<usize>>]) -> bool {
    connections[*tuple.0].as_ref().unwrap().contains(tuple.1)
        && connections[*tuple.0].as_ref().unwrap().contains(tuple.2)
        && connections[*tuple.0].as_ref().unwrap().contains(tuple.3)
        && connections[*tuple.0].as_ref().unwrap().contains(tuple.4)
        && connections[*tuple.0].as_ref().unwrap().contains(tuple.5)
        && connections[*tuple.1].as_ref().unwrap().contains(tuple.2)
        && connections[*tuple.1].as_ref().unwrap().contains(tuple.3)
        && connections[*tuple.1].as_ref().unwrap().contains(tuple.4)
        && connections[*tuple.1].as_ref().unwrap().contains(tuple.5)
        && connections[*tuple.2].as_ref().unwrap().contains(tuple.3)
        && connections[*tuple.2].as_ref().unwrap().contains(tuple.4)
        && connections[*tuple.2].as_ref().unwrap().contains(tuple.5)
        && connections[*tuple.3].as_ref().unwrap().contains(tuple.4)
        && connections[*tuple.3].as_ref().unwrap().contains(tuple.5)
        && connections[*tuple.4].as_ref().unwrap().contains(tuple.5)
}

type FiveTuple<'a> = (&'a usize, &'a usize, &'a usize, &'a usize, &'a usize);

pub fn five(tuple: FiveTuple, connections: &[Option<Vec<usize>>]) -> bool {
    connections[*tuple.0].as_ref().unwrap().contains(tuple.1)
        && connections[*tuple.0].as_ref().unwrap().contains(tuple.2)
        && connections[*tuple.0].as_ref().unwrap().contains(tuple.3)
        && connections[*tuple.0].as_ref().unwrap().contains(tuple.4)
        && connections[*tuple.1].as_ref().unwrap().contains(tuple.2)
        && connections[*tuple.1].as_ref().unwrap().contains(tuple.3)
        && connections[*tuple.1].as_ref().unwrap().contains(tuple.4)
        && connections[*tuple.2].as_ref().unwrap().contains(tuple.3)
        && connections[*tuple.2].as_ref().unwrap().contains(tuple.4)
        && connections[*tuple.3].as_ref().unwrap().contains(tuple.4)
}

type FourTuple<'a> = (&'a usize, &'a usize, &'a usize, &'a usize);

pub fn four(tuple: FourTuple, connections: &[Option<Vec<usize>>]) -> bool {
    connections[*tuple.0].as_ref().unwrap().contains(tuple.1)
        && connections[*tuple.0].as_ref().unwrap().contains(tuple.2)
        && connections[*tuple.0].as_ref().unwrap().contains(tuple.3)
        && connections[*tuple.1].as_ref().unwrap().contains(tuple.2)
        && connections[*tuple.1].as_ref().unwrap().contains(tuple.3)
        && connections[*tuple.2].as_ref().unwrap().contains(tuple.3)
}

type ThreeTuple<'a> = (&'a usize, &'a usize, &'a usize);

pub fn three(tuple: ThreeTuple, connections: &[Option<Vec<usize>>]) -> bool {
    connections[*tuple.0].as_ref().unwrap().contains(tuple.1)
        && connections[*tuple.0].as_ref().unwrap().contains(tuple.2)
        && connections[*tuple.1].as_ref().unwrap().contains(tuple.2)
}
