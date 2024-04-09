use polars::prelude as pl;
use polars::lazy::{read_csv, read_parquet};

pub struct DataSet {
    lf: &pl::LazyFrame,
    node_columns: Vec<String>,
    edge_columns: Vec<String>,
    weight_column: String,
}

impl DataSet {
    pub fn from_csv(
        path: &str,
        node_columns: Vec<String>,
        edge_columns: Vec<String>,
        weight_column: String,
    ) -> Self {
        let lf = LazyCSVReader::new(&path)
            .raise_if_empty()
            .with_missing_is_null()
            .has_header(true)

            path,
        )
        DataSet {
            &lf,
            node_columns,
            edge_columns,
            weight_column,
        }
    }

    pub fn from_parquet(
        path: &str,
        node_columns: Vec<String>,
        edge_columns: Vec<String>,
        weight_column: String,
    ) -> Self {
        let lf = pl::lazy::read_parquet(path).unwrap();
        DataSet {
            lf,
            node_columns,
            edge_columns,
            weight_column,
        }
    }

    pub fn get_lf(&self) -> &pl::LazyFrame {
        &self.lf
    }

    pub fn set_lf(&mut self, lf: pl::LazyFrame) {
        self.lf = lf;
    }

    pub fn get_weight_col(&self) -> pl::LazyFrame {
        &self.lf.select([self.weight_column.as_str()])
    }

    pub fn get_unique(&self, cols: Vec<&str>) -> pl::LazyFrame {
        &self.lf.select([cols.iter().map(|c| c.as_str())]).unique()
    }

    pub fn get_nodes(&self) -> pl::LazyFrame {
        &self.get_unique(self.node_columns)
    }

    pub fn get_edges(&self) -> pl::LazyFrame {
        &self.get_unique(self.edge_columns)
    }
}
