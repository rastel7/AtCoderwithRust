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
    fn get(&self, x: T) -> Option<usize> {
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
