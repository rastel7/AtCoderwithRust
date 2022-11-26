use super::edge::Edge;
use std::collections::BinaryHeap;
#[allow(unused)]
struct Dijkstra<T: Ord + Clone + std::ops::Add> {
    graph: Vec<Vec<Edge<T>>>,
}
/* 
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
*/
#[allow(unused)]
impl Dijkstra<i64> {
    pub fn new(edges: Vec<Edge<i64>>) -> Self {
        let n = Edge::max_id(&edges);
        let mut v: Vec<Vec<Edge<i64>>> = vec![Vec::new(); n + 1];
        for e in edges {
            v[e.from].push(e);
        }
        Dijkstra { graph: (v) }
    }
    fn query(&self, s: usize) -> Vec<Option<i64>> {
        let mut dist: Vec<Option<i64>> = vec![None; self.graph.len()];
        let mut binheap: BinaryHeap<(Option<i64>, usize)> = BinaryHeap::new();
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
        dist[s]=Some(0);
        dist
    }
}