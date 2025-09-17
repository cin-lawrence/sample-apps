#[allow(unused)]
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd)]
pub enum Sort {
    Ascending,
    Descending,
}

impl std::fmt::Display for Sort {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Sort::Ascending => write!(f, "asc"),
            Sort::Descending => write!(f, "desc"),
        }
    }
}
