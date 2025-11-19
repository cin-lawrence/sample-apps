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

#[derive(Default)]
pub enum Size {
    Small,
    #[default]
    Medium,
    Large,
}

impl std::fmt::Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Size::Small => "small".fmt(f),
            Size::Medium => "medium".fmt(f),
            Size::Large => "large".fmt(f),
        }
    }
}

impl std::str::FromStr for Size {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Size::*;
        match s.to_lowercase().as_str() {
            "small" => Ok(Small),
            "medium" => Ok(Medium),
            "large" => Ok(Large),
            _ => Err(()),
        }
    }
}
