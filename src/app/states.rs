use std::{collections::HashMap, fmt::Display};

use serde::{Deserialize, Serialize};
use yewdux::store::Store;

#[derive(Default, Clone, PartialEq, Eq, Deserialize, Serialize, Store)]
#[store(storage = "local")]
pub(crate) struct Username(pub(crate) String);

impl Display for Username {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Store)]
pub(crate) struct CollisionState {
    pub(crate) on_collision_stay: bool,
    pub(crate) url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Store)]
pub(crate) struct ChatTextState {
    pub(crate) message: String,
    pub(crate) is_display_balloon: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Store)]
pub(crate) struct ChatTextHashState {
    pub(crate) hash: HashMap<String, ChatTextState>,
}

impl ChatTextHashState {
    pub(crate) fn get(&self, key: &str) -> Option<&ChatTextState> {
        self.hash.get(key)
    }
}
