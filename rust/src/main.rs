extern crate time;

use std::rand::{task_rng, Rng};
use std::mem::replace;
use time::precise_time_ns;

#[deriving(Show, Clone)]
struct Data {
    a: uint,
    b: uint,
    c: uint
}

#[deriving(Show, Clone)]
enum Row {
    Cons(Data, Box<Row>),
    Nil
}

impl Row {
    fn new(d: Data) -> Row {
        Cons(d, box Nil)
    }

    fn prepend(self, d: Data) -> Row {
        Cons(d, box self)
    }

    fn find<'a>(&'a self, a: uint) -> Option<&'a Data> {
        match *self {
            Cons(ref data, ref tl) => {
                if data.a == a { return Some(data); }
                return tl.find(a)
            }
            Nil => return None
        };
    }
}

#[deriving(Show)]
struct Relation {
    size: uint,
    size_index: uint,
    index: Vec<Box<Row>>
}

impl Relation {
    fn new(size_index: uint) -> Relation {
        Relation {
            size: 0,
            size_index: size_index,
            index: Vec::from_fn(size_index, |_| box Nil)
        }
    }

    fn hash(&self, a: uint) -> uint {
        a & (self.size_index-1)
    }

    fn insert(&mut self, a: uint, b: uint, c: uint) {
        self.size += 1;
        let hash = self.hash(a);
        let data = Data {a: a, b: b, c: c};

        let prepend = match **self.index.get(hash) {
            Cons(..) => true,
            Nil      => {
                *self.index.get_mut(hash) = box Row::new(data);
                false
            }
        };

        if prepend {
            let old_row = replace(self.index.get_mut(hash), box Nil);
            replace(self.index.get_mut(hash), box old_row.prepend(data));
        }
    }

    fn lookup<'a>(&'a self, a: uint) -> Option<&'a Data> {
        self.index.get(self.hash(a)).find(a)
    }
}


fn main() {
    let n: uint = 2500000;
    let mut rel = Relation::new(1u << 20);
    let mut v: Vec<Row> = Vec::with_capacity(n);

    for i in range(0, n) {
        v.push(Row::new(Data {a: i, b: i/3, c: i/7}));
    }

    let mut rng = task_rng();
    rng.shuffle(v.as_mut_slice());

    let mut time = precise_time_ns();
    for row in v.iter() {
        match *row {
            Cons(d, _) => rel.insert(d.a, d.b, d.c),
            Nil        => ()
        }
    }
    println!("insert: {} s", ((precise_time_ns() - time) as f64) / 1e9f64);


    rng.shuffle(v.as_mut_slice());

    time = precise_time_ns();
    for r in v.iter() {
        match *r {
            Cons(d, _) => {
                let r2 = rel.lookup(d.a);
                match r2 {
                    Some(d2) => {
                        assert!(rel.hash(d.a) == rel.hash(d2.a));
                        assert!(d.a == d2.a);
                    },
                    None     => assert!(false)
                }
            },
            Nil => assert!(false)
        }
    }
    println!("lookup: {} s", ((precise_time_ns() - time) as f64) / 1e9f64);
}
