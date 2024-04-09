struct CoverageNodeSet {
    c1: String,
    c2: String,
}

impl CoverageNodeSet {
    fn new(c1: &str, c2: &str) -> Self {
        CoverageNodeSet {
            c1: c1.to_owned(),
            c2: c2.to_owned(),
        }
    }

    fn get_c1(&self) -> &str {
        &self.c1
    }

    fn get_c2(&self) -> &str {
        &self.c2
    }

    fn set_c1(&mut self, c1: &str) {
        self.c1 = c1.to_owned();
    }

    fn set_c2(&mut self, c2: &str) {
        self.c2 = c2.to_owned();
    }

    /// Compare two coverage sets
    ///
    /// Two coverage sets are considered equal if:
    /// - c1 of self is equal to c1 of other and c2 of self is equal to c2 of other
    /// OR
    /// - c1 of self is equal to c2 of other and c2 of self is equal to c1 of other
    fn eq(&self, other: CoverageNodeSet) -> bool {
        (self.get_c1() == other.get_c1() && self.get_c2() == other.get_c2())
            || (self.get_c1() == other.get_c2() && self.get_c2() == other.get_c1())
    }
}

struct CoverageNodes {
    nodes: Vec<CoverageNodeSet>,
}

impl CoverageNodes {
    fn already_exists(&self, c1: &str, c2: &str) -> bool {
        self.nodes
            .iter()
            .any(|n| n.eq(CoverageNodeSet::new(c1, c2)))
    }

    fn new(&mut self, c1: &str, c2: &str) -> () {
        if !self.already_exists(c1, c2) {
            self.nodes.push(CoverageNodeSet::new(c1, c2));
        }
    }
}
