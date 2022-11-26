#[allow(unused_imports)]
use proconio::{input, marker::Chars};

use std::collections::BinaryHeap;

use std::cmp::Ordering;
#[derive(Eq, Clone, Copy)]
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
#[allow(unused)]
struct Dijkstra<T: Ord + Clone + std::ops::Add> {
    graph: Vec<Vec<Edge<T>>>,
}

#[allow(unused)]
impl<T: Ord + Clone + std::ops::Add<Output = T>> Dijkstra<T> {
    pub fn new(edges: Vec<Edge<T>>) -> Self {
        let n = Edge::max_id(&edges);
        let mut v: Vec<Vec<Edge<T>>> = vec![Vec::new(); n + 1];
        for e in edges {
            v[e.from].push(e);
        }
        Dijkstra { graph: (v) }
    }
    fn query(&self, s: usize) -> Vec<Option<T>> {
        let mut dist: Vec<Option<T>> = vec![None; self.graph.len()];
        let mut binheap: BinaryHeap<(Option<T>, usize)> = BinaryHeap::new();
        binheap.push((dist[s].clone(), s));
        while !binheap.is_empty() {
            let (cost, id) = binheap.pop().unwrap();
            for edge in &self.graph[id] {
                let next = (match cost.clone() {
                    None => edge.cost.clone(),
                    Some(w) => (edge.cost.clone() + w.clone()),
                });

                match &dist[edge.to] {
                    None => {
                        dist[edge.to] = Some(next.clone());
                    }
                    Some(w) => {
                        if w <= &next {
                            continue;
                        }
                        dist[edge.to] = Some(next.clone());
                    }
                }
                binheap.push((Some(next), edge.to));
            }
        }
        dist
    }
}

fn main() {
    input! {
       n:usize,
       s:[i64;n],
       t:[i64;n],
    }
    let mut edges: Vec<Edge<i64>> = Vec::new();
    for i in 0..n {
        edges.push(Edge::new(n, i, t[i]));
        edges.push(Edge::new(i, (i + 1) % n, s[i]));
    }
    let mut dijkstra = Dijkstra::new(edges);
    let mut ret = dijkstra.query(n);
    for i in 0..n{
        println!("{}",ret[i].unwrap());
    }
}
