
#[derive(Debug)]
pub struct TableError {
    description: String,
}

impl TableError {
    pub fn new<S: Into<String>>(s: S) -> TableError {
        TableError {
            description: s.into(),
        }
    }
}

impl std::fmt::Display for TableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description)
    }
}
