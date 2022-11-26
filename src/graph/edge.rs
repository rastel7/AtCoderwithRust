use std::cmp::Ordering;
#[derive(Eq,Clone,Copy)]
pub struct Edge<T: Ord + Clone> {
  pub from: usize,
  pub to: usize,
  pub cost: T,
}
#[allow(unused)]
impl<T: Ord + Clone> Edge<T> {
    pub fn new(from: usize, to: usize, cost: T) -> Self {
        Edge {
            from: from,
            to: to,
            cost: cost,
        }
    }
    pub fn max_id(edges: &Vec<Edge<T>>) -> usize {
        let mut ret = 0;
        for e in edges {
            ret = std::cmp::max(ret, e.from);
            ret = std::cmp::max(ret, e.to);
        }
        ret
    }
}

impl<T: Ord + Clone> Ord for Edge<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.cost != other.cost {
            return self.cost.cmp(&other.cost);
        } else if self.from != other.from {
            return self.from.cmp(&other.from);
        } else {
            return self.to.cmp(&other.to);
        }
    }
}

impl<T: Ord + Clone> PartialOrd for Edge<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Ord + Clone> PartialEq for Edge<T> {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost && self.from == other.from && self.to == other.to
    }
}

#[test]
fn it_works() {
    let e1 = Edge::new(0, 1, 1);
    let e2 = Edge::new(0, 1, 1);
    assert_eq!(e1 == e2, true);
    let e1 = Edge::new(0, 1, 1);
    let e2 = Edge::new(1, 2, 2);
    assert_eq!(e1 < e2, true);
}
