
pub mod gs {
    pub type GSI = u32; // Graph Space Integer
    pub struct Digraph(Vec<(GSI, GSI)>);
    impl Digraph {
        pub fn new() -> Self {Digraph(Vec::new())}
        pub fn edges(&self, a: Option<GSI>, b: Option<GSI>) -> Box<dyn Iterator<Item=&(GSI, GSI)> + '_> {
            let mut itr: Box<dyn Iterator<Item=&(GSI, GSI)>> = Box::new(self.0.iter());
            if let Some(y) = a {
                itr = Box::new(itr.filter(move |x| x.0 == y));
            }
            if let Some(y) = b {
                itr = Box::new(itr.filter(move |x| x.1 == y));
            }
            return itr;
        }
        pub fn add_edge(&mut self, a: GSI, b: GSI) {
            self.0.push((a, b));
        }
        pub fn remove_edges(&mut self, a: Option<GSI>, b: Option<GSI>) {
            match (a, b) {
                (Some(a), Some(b)) => {
                    self.0.retain(move |&(x, y)| x != a && y != b);
                }
                (Some(a), None) => {
                    self.0.retain(move |&(x,_)| x != a);
                }
                (None, Some(b)) => {
                    self.0.retain(move |&(_, y)| y != b);
                }
                (None, None) => ()
            }
        }
        pub fn inner(&self) -> &Vec<(GSI, GSI)> { &self.0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn prep_digraph() -> gs::Digraph {
        let mut digraph = gs::Digraph::new();
        // 5 nodes [0 .. 5]
        digraph.add_edge(0, 1);
        digraph.add_edge(1, 2);
        digraph.add_edge(2, 4);
        digraph.add_edge(4, 1);
        digraph.add_edge(3, 2);
        digraph.add_edge(0, 3);
        digraph.add_edge(0, 2);
        return digraph;
    }
    
    // extracting
    #[test]
    fn digraph_find_ends() {
        let digraph = prep_digraph();
        let mut itr = digraph.edges(None, Some(0));
        assert_eq!(itr.next(), None);
    }
    #[test]
    fn digraph_find_single() {
        let digraph = prep_digraph();
        let mut itr = digraph.edges(Some(0), Some(3));
        assert_eq!(itr.next(), Some(&(0, 3))); 
    }
    #[test]
    fn digraph_find_starts() {
        let digraph = prep_digraph();
        let mut itr = digraph.edges(Some(0), None);
        assert_eq!(itr.next(), Some(&(0, 1)));
        assert_eq!(itr.next(), Some(&(0, 3)));
        assert_eq!(itr.next(), Some(&(0, 2)));
        assert_eq!(itr.next(), None);
    }
    #[test]
    fn digraph_find_all() {
        let digraph = prep_digraph();
        for (c, f) in digraph.inner().iter().zip(digraph.edges(None, None)) {
            assert_eq!(c, f);
        }
    }
    // removing
    #[test]
    fn digraph_remove_single() {
        let mut digraph = prep_digraph();
        digraph.remove_edges(Some(0), Some(3));
        let mut itr = digraph.edges(Some(0), Some(3));
        assert_eq!(itr.next(), None);
    }
    #[test]
    fn digraph_remove_end() {
        let mut digraph = prep_digraph();
        digraph.remove_edges(None, Some(3));
        println!("{:?}", digraph.inner());
        let mut itr = digraph.edges(None, Some(3));
        assert_eq!(itr.next(), None);
    }
    #[test]
    fn digraph_remove_start() {
        let mut digraph = prep_digraph();
        digraph.remove_edges(Some(0), None);
        let mut itr = digraph.edges(Some(0), None);
        assert_eq!(itr.next(), None);
    }
}
