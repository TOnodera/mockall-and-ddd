use serde::{Deserialize, Serialize};
use std::fmt::Display;

use super::name_eq;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, Deserialize)]
pub struct UserLastName(String);

impl UserLastName {
    pub fn new(last_name: impl ToString) -> Self {
        Self(last_name.to_string())
    }
    pub fn eq(&self, last_name: &Self) -> bool {
        name_eq(self.0.as_str(), last_name.0.as_str())
    }
}

impl Display for UserLastName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
