use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub type CrdtState = Arc<RwLock<State>>;

#[derive(Default)]
pub struct State {
    pub db: HashMap<String, String>,
}
