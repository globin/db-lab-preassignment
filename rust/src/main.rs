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

    fn find(&self, a: uint) -> Option<&Data> {
        match *self {
            Cons(ref data, ref tl) => {
                if data.a == a { return Some(data); }
                return tl.find(a)
            }
            Nil => return None
        };
    }

    fn next(&self) -> Option<&Row> {
        match *self {
            Cons(_, ref next) => Some(&**next),
            Nil => None
        }
    }

    fn data(&self) -> &Data {
        match *self {
            Cons(ref data, _) => data,
            Nil => fail!("fnord")
        }
    }

    fn remove(&mut self, a: uint) {
        let tmp = &mut box Nil;
        let mut swap = false;

        match *self {
            Cons(ref data, ref mut next) => {
                if data.a == a {
                    swap = true;
                    mem::swap(next, tmp)
                } else {
                    next.remove(a)
                }
            }
            Nil => ()
        }

        if swap {
            mem::swap(self, &mut **tmp);
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
            index: Vec::from_elem(size_index, Nil)
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

    fn lookup(&self, a: uint) -> Option<&Data> {
        self.index[self.hash(a)].find(a)
    }

    fn remove(&mut self, data: &Data) {
        let hash = self.hash(data.a);

        self.index.get_mut(hash).remove(data.a);
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
    println!("insert {}s", ((precise_time_ns() - time) as f64) / 1e9f64);


    rng.shuffle(v.as_mut_slice());

    time = precise_time_ns();
    for d in v.iter() {
        match rel.lookup(d.a){
            Some(d2) => assert_eq!(d.a, d2.a),
            None     => assert!(false)
        }
    }
    println!("lookup {}s", ((precise_time_ns() - time) as f64) / 1e9f64);


    let mut sum = 0u;
    time = precise_time_ns();
    for row in rel.index.iter() {
        match *row {
            Cons(ref data, ref next) => {
                sum += data.a;
                let mut iter_row = &**next;
                while iter_row.next().is_some() {
                    sum += iter_row.data().a;
                    iter_row = iter_row.next().unwrap();
                }
            },
            Nil => ()
        }
    }
    println!("scan {}s", ((precise_time_ns() - time) as f64) / 1e9f64);
    assert_eq!(n*(n-1)/2, sum);


    rng.shuffle(v.as_mut_slice());

    time = precise_time_ns();
    for d in v.iter() {
        assert!(rel.lookup(d.a).is_some());
        rel.remove(d);
        assert!(rel.lookup(d.a).is_none());
    }
    println!("remove {}s", ((precise_time_ns() - time) as f64) / 1e9f64);

    assert!(rel.index.iter().all(|item| match *item {
        Cons(..) => false,
        Nil => true
    }));
}
