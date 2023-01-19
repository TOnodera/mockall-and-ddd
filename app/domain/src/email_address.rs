use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct EmailAddress {
    user: String,
    domain: String,
}

impl EmailAddress {
    pub fn new(s: impl ToString) -> Self {
        // TODO ここでバリューオブジェクトのバリデーション行う
        let s = s.to_string();
        let mut sp = s.split('@');

        // TODO 想定されるエラーなのでエラー型を返すようにしたい
        Self {
            user: sp
                .next()
                .expect("メールアドレスのフォーマットがおかしいです。")
                .to_string(),
            domain: sp
                .next()
                .expect("メールアドレスのフォーマットがおかしいです。")
                .to_string(),
        }
    }
}

impl Display for EmailAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}@{}", self.user, self.domain)
    }
}
