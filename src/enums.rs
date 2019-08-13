#[derive(Debug, PartialEq)]
pub enum IndexOrColumn {
    Column(String),
    Index(usize),
}

impl From<String> for IndexOrColumn {
    fn from(s: String) -> Self {
        IndexOrColumn::Column(s)
    }
}

impl From<&str> for IndexOrColumn {
    fn from(s: &str) -> Self {
        IndexOrColumn::Column(s.to_string())
    }
}

// impl<S: Into<String>> From<S> for IndexOrColumn{
//     fn from(s: S) -> Self{
//         IndexOrColumn::Column(s.into())
//     }
// }

impl From<usize> for IndexOrColumn {
    fn from(s: usize) -> Self {
        IndexOrColumn::Index(s)
    }
}
