extern crate time;

use std::rand::{task_rng, Rng};
use std::mem;
use time::precise_time_ns;

#[deriving(Show, Clone)]
struct Data {
    a: uint,
    b: uint,
    c: uint,
}

#[deriving(Show, Clone)]
enum Row {
    Cons(Data, Box<Row>),
    Nil,
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

    fn remove(row: &mut Row, a: uint) {
        let tmp = &mut box Nil;
        let mut swap = false;

        match *row {
            Cons(ref data, ref mut next) => {
                if data.a == a {
                    swap = true;
                    mem::swap(next, tmp)
                }
                else { Row::remove(&mut **next, a); }
            }
            Nil => ()
        }

        if swap {
            mem::swap(row, &mut **tmp);
        }
    }
}

#[deriving(Show)]
struct Relation {
    size: uint,
    size_index: uint,
    index: Vec<Row>,
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

        let prepend = match self.index[hash] {
            Cons(..) => true,
            Nil      => {
                *self.index.get_mut(hash) = Row::new(data);
                false
            }
        };

        if prepend {
            let old_row = mem::replace(self.index.get_mut(hash), Nil);
            mem::swap(self.index.get_mut(hash), &mut old_row.prepend(data));
        }
    }

    fn lookup<'a>(&'a self, a: uint) -> Option<&'a Data> {
        self.index[self.hash(a)].find(a)
    }

    fn remove(&mut self, data: &Data) {
        let hash = self.hash(data.a);

        Row::remove(self.index.get_mut(hash), data.a);
    }
}

fn main() {
    let n: uint = 2500000;
    let mut rel = Relation::new(1u << 20);
    let mut v: Vec<Data> = Vec::with_capacity(n);

    for i in range(0, n) {
        v.push(Data {a: i, b: i/3, c: i/7});
    }

    let mut rng = task_rng();
    rng.shuffle(v.as_mut_slice());

    let mut time = precise_time_ns();
    for d in v.iter() {
        rel.insert(d.a, d.b, d.c)
    }
    println!("insert: {} s", ((precise_time_ns() - time) as f64) / 1e9f64);


    rng.shuffle(v.as_mut_slice());

    time = precise_time_ns();
    for d in v.iter() {
        match rel.lookup(d.a){
            Some(d2) => assert!(d.a == d2.a),
            None     => assert!(false)
        }
    }
    println!("lookup: {} s", ((precise_time_ns() - time) as f64) / 1e9f64);


    // TODO: scan


    rng.shuffle(v.as_mut_slice());

    time = precise_time_ns();
    for d in v.iter() {
        rel.lookup(d.a).unwrap();
        rel.remove(d);
        match rel.lookup(d.a){
            Some(..) => assert!(false),
            None     => assert!(true)
        }
    }
    println!("remove: {} s", ((precise_time_ns() - time) as f64) / 1e9f64);
    // TODO: check if all Nil
}
