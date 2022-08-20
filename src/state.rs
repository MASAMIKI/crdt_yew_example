use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Rc;
use yew::prelude::*;

pub enum Action {
    Edit(String, String),
}
/// state for values of input form
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct State {
    pub hash_map: HashMap<String, String>,
}

impl Reducible for State {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            Action::Edit(key, value) => {
                let mut hash_map = self.hash_map.clone();
                hash_map.insert(key, value);
                State { hash_map }.into()
            }
        }
    }
}
