use std::{
    cmp,
    sync::{
        atomic::{self, AtomicU32},
        Arc,
    },
};

use serde::Serialize;

#[derive(Hash, Serialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Player {
    id: u32,
}

#[allow(non_camel_case_types)]
pub type PlayerId_t = u32;

static IDGEN: AtomicU32 = AtomicU32::new(0);

impl Player {
    pub fn new() -> Self {
        Player {
            id: Self::get_new_id(),
        }
    }
    fn get_new_id() -> PlayerId_t {
        IDGEN.fetch_add(1, atomic::Ordering::SeqCst)
    }
    pub fn id(&self) -> PlayerId_t {
        self.id
    }
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}
