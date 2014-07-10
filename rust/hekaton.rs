extern crate time;

use std::rand::{task_rng, Rng};
use time::precise_time_ns;

#[deriving(Show)]
struct Data {
    a: uint,
    b: uint,
    c: uint
}

#[deriving(Show)]
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

    //fn a(self) -> uint {
        //let a : Option<uint> = match self {
            //Cons(a, _) => Some(a),
            //_ => None
        //};
        //a.unwrap()
    //}
}

#[deriving(Show)]
struct Relation {
    size: uint,
    size_index: uint,
    index: Vec<Row>
}

impl Relation {
    fn new(size_index: uint) -> Relation {
        Relation {
            size: 0,
            size_index: size_index,
            index: Vec::from_fn(size_index, |_| Nil)
        }
    }

    fn hash(&self, a: uint) -> uint {
        a & (self.size_index-1)
    }

    fn insert(&mut self, a: uint, b: uint, c: uint) {
        self.size += 1;
        let hash = self.hash(a);
        let data = Data {a: a, b: b, c: c};

        match *self.index.get(hash) {
            Cons(..) => (),
            Nil      => { *self.index.get_mut(hash) = Row::new(data); }
        }
    }

    fn lookup<'a>(&'a self, a: uint) -> &'a Row {
        self.index.get(self.hash(a))
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
    println!("insert: {} ms", ((precise_time_ns() - time) as f64) / 1e6f64);


    rng.shuffle(v.as_mut_slice());

    time = precise_time_ns();
    for r in v.iter() {
        match *r {
            Cons(d, _) => {
                let r2 = rel.lookup(d.a);
                match *r2 {
                    Cons(d2, _) => {
                        //if d.a != d2.a { println!("{} == {}", rel.hash(d.a), rel.hash(d2.a)); }
                        assert!(rel.hash(d.a) == rel.hash(d2.a));
                        //assert!(d.a == d2.a);
                    },
                    Nil         => assert!(false)
                }
            },
            Nil => assert!(false)
        }
    }
    println!("lookup: {} s", ((precise_time_ns() - time) as f64) / 1e9f64);
}
