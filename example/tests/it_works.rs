use sea_orm::{EnumIter, Iterable};

#[example::example(table = "example", kind(type = "Kind", fn = "from_str"))]
#[derive(Debug, EnumIter)]
pub enum Example {}

#[derive(Debug)]
pub enum Kind {
    Default,
    Custom,
}

impl Kind {
    const fn from_str(s: &'static str) -> Self {
        match s.as_bytes() {
            b"D" => Kind::Default,
            b"C" => Kind::Custom,
            _ => unreachable!(),
        }
    }
}

#[test]
fn it_works() {
    for ex in Example::iter() {
        println!("name: {}, kind: {:?}", ex.as_str(), ex.kind());
    }
}
