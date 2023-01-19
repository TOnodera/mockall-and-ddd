use serde::{Deserialize, Serialize};
use std::fmt::Display;

use super::name_eq;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, Deserialize)]
pub struct UserFirstName(String);

impl UserFirstName {
    pub fn new(first_name: impl ToString) -> Self {
        Self(first_name.to_string())
    }
    pub fn eq(&self, first_name: &Self) -> bool {
        name_eq(self.0.as_str(), first_name.0.as_str())
    }
}

impl Display for UserFirstName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
