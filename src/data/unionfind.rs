#![allow(unused_results, dead_code)]
struct UnionFind {
    tops: Vec<usize>,
    size: Vec<usize>,
}
#[allow(dead_code)]
impl UnionFind {
    fn new(n: usize) -> Self {
        let mut ret = Self {
            tops: vec![0; n],
            size: vec![0; n],
        };
        for i in 0..n {
            ret.tops[i] = i;
        }
        ret
    }
    fn root(&mut self, x: usize) -> usize {
        if self.tops[x] == x {
            x
        } else {
            let rt = self.root(self.tops[x]);
            self.tops[x] = rt;
            rt
        }
    }
    fn merge(&mut self, x: usize, y: usize) {
        let rtx = self.root(x);
        let rty = self.root(y);
        if rtx == rty {
            return;
        }
        if self.size[rtx] < self.size[rty] {
            self.tops[rtx] = rty;
        } else {
            self.tops[rty] = rtx;
            if self.size[rtx] == self.size[rty] {
                self.size[rtx] += 1;
            }
        }
    }
    fn issame(&mut self, x: usize, y: usize) -> bool {
        let rx = self.tops[x];
        let ry = self.tops[y];
        let rx = self.root(rx);
        let ry = self.root(ry);
        rx == ry
    }
}

#[test]
fn it_works() {
    let n = 5;
    let mut uf = UnionFind::new(n);
    uf.merge(0, 1);
    uf.merge(2, 3);
    assert_eq!(uf.issame(0, 1), true);
    assert_eq!(uf.issame(2, 3), true);
    assert_eq!(uf.issame(0, 2), false);
}
