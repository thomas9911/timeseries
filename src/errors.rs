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

#[derive(Debug)]
pub struct TableReadError(String);

impl TableReadError {
    pub fn new<S: Into<String>>(s: S) -> TableReadError {
        TableReadError { 0: s.into() }
    }
}

impl std::fmt::Display for TableReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(feature = "csv")]
impl From<csv::Error> for TableReadError {
    fn from(err: csv::Error) -> Self {
        Self::new(format!("{}", err))
    }
}

impl From<crate::TableError> for TableReadError {
    fn from(err: crate::TableError) -> Self {
        Self::new(format!("{}", err))
    }
}

#[cfg(feature = "chrono")]
impl From<chrono::ParseError> for TableReadError {
    fn from(err: chrono::ParseError) -> Self {
        Self::new(format!("{}", err))
    }
}
