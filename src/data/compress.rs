use std::collections::HashMap;
use std::hash::Hash;
#[allow(unused)]
struct Compress<T: Clone> {
    tops: Vec<T>,
    maps: HashMap<T, usize>,
}
#[allow(unused)]
impl<T: Clone + Ord + Hash + Copy> Compress<T> {
    fn new(t: Vec<T>) -> Self {
        let mut ret = Self {
            tops: t.clone(),
            maps: HashMap::new(),
        };
        ret.tops.sort();
        ret.tops.dedup();
        for i in 0..ret.tops.len() {
            ret.maps.insert(ret.tops[i], i);
        }
        ret
    }
    fn f(&self, x: T) -> Option<usize> {
        match self.maps.get(&x) {
            None => None,
            Some(w) => Some(w.clone()),
        }
    }
    fn invf(&self, id: usize) -> Option<T> {
        if self.tops.len() > id {
            Some(self.tops[id])
        } else {
            None
        }
    }
}

#[test]
fn it_works() {
    let vec = vec![0, 1, 4, 5, 6, 8, 10];
    let  comp = Compress::new(vec);
    assert_eq!(comp.f(0), Some(0));
    assert_eq!(comp.f(5), Some(3));
    assert_eq!(comp.invf(6), Some(10));
    assert_eq!(comp.invf(4), Some(6));
}
