use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    time::Duration,
};

type Db = Arc<RwLock<HashMap<Uuid, Todo>>>;
