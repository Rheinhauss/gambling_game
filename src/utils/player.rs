use std::{
    cmp,
    sync::{
        atomic::{self, AtomicU32},
        Arc,
    },
};

#[derive(Hash)]
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
impl PartialOrd for Player {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.id.cmp(&other.id))
    }
}
impl Ord for Player {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.id.cmp(&other.id)
    }
}
impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Player {}

impl Clone for Player {
    fn clone(&self) -> Self {
        Player { id: self.id }
    }
}
impl Copy for Player {}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl std::fmt::Debug for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[player]{}", self.id)
    }
}
