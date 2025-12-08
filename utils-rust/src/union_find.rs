pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    #[must_use]
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: std::iter::repeat_n(0, n).collect(),
            size: std::iter::repeat_n(1, n).collect(),
        }
    }

    #[must_use]
    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    pub fn union(&mut self, a: usize, b: usize) -> usize {
        let mut a = self.find(a);
        let mut b = self.find(b);
        if a != b {
            if self.rank[a] < self.rank[b] {
                (a, b) = (b, a);
            }
            self.parent[b] = a;
            if self.rank[a] == self.rank[b] {
                self.rank[a] += 1;
            }
            self.size[a] += self.size[b];
            self.size[b] = 0;
        }
        a
    }

    #[must_use]
    pub fn sizes(&self) -> &[usize] {
        &self.size
    }
}
