use std::num::ParseIntError;
use std::str::FromStr;
use std::fmt::{Display, Formatter};


#[derive(PartialEq, Clone, Debug, Default)]
pub struct PreviewState {
    pub active_story: Option<i64>,
}

impl FromStr for PreviewState {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let state = i64::from_str(s)?;
        Ok(PreviewState {
            active_story: Some(state),
        })
    }
}

impl Display for PreviewState {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        if let Some(id) = &self.active_story {
            write!(f, "{id}")?;
        }
        Ok(())
    }
}
