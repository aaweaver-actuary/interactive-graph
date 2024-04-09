use bop_coverage_graph::data_io::DataSet;
use bop_coverage_graph::edge::CoverageEdges;
use bop_coverage_graph::node::CoverageNodes;
use petgraph::graphmap::UnGraphMap;

pub struct Graph<N, E, Ty> {
    data: DataSet,
    graph: UnGraphMap<N, E, Ty>,
}

impl Graph {
    pub fn new(data: DataSet, n_edges: usize, n_nodes: usize) -> Self {
        Graph {
            data,
            graph: UnGraphMap::with_capacity(n_edges, n_nodes),
        }
    }

    pub fn from_csv(
        path: &str,
        node_columns: Vec<&str>,
        edge_columns: Vec<&str>,
        weight_column: &str,
    ) -> Self {
        let data = DataSet::from_csv(path, node_columns, edge_columns, weight_column);
        let n_nodes = data
            .lf
            .select(node_columns.iter().map(|c| c.as_str()))
            .unique()
            .nrows();
        let n_edges = data
            .lf
            .select(edge_columns.iter().map(|c| c.as_str()))
            .unique()
            .nrows();

        Graph::new(data, n_edges, n_nodes)
    }

    pub fn from_parquet(
        path: &str,
        node_columns: Vec<&str>,
        edge_columns: Vec<&str>,
        weight_column: &str,
    ) -> Self {
        let data = DataSet::from_parquet(path, node_columns, edge_columns, weight_column);
        let n_nodes = data
            .lf
            .select(node_columns.iter().map(|c| c.as_str()))
            .unique()
            .nrows();
        let n_edges = data
            .lf
            .select(edge_columns.iter().map(|c| c.as_str()))
            .unique()
            .nrows();

        Graph::new(data, n_edges, n_nodes)
    }

    pub fn nodes(&self) -> Vec<&str> {
        &self.data.lf.select([self.data.node_columns.iter().map(|c| c.as_str())])
    }
    }
}

impl Iterator for Graph {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        None
    }


}
