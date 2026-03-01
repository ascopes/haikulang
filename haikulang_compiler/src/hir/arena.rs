use std::collections::HashMap;
use std::hash::Hash;

/// Alias to the arena implementation so that we can more easily swap it out later.
///
/// The arena is a type that takes ownership of objects we define and provides us an ID
/// handle to reference those objects elsewhere, allowing us to flatten trees and deduplicate
/// certain constructs like strings.
pub type Arena<T> = la_arena::Arena<T>;

/// Alias to the arena ID implementation so we can more easily swap it out later.
pub type Id<T> = la_arena::Idx<T>;

/// Helper data structure that enables interning special types like Strings so we only keep them
/// in memory once. This also allows us to only declare special values in data sections later once
/// when their values may be duplicated.
#[derive(Clone, Debug)]
pub struct InterningArena<T: Clone + Eq + Hash> {
    arena: Arena<T>,
    id_mapping: HashMap<T, Id<T>>,
}

impl<T: Clone + Eq + Hash> InterningArena<T> {
    pub fn new() -> Self {
        Self {
            arena: Arena::new(),
            id_mapping: HashMap::new(),
        }
    }

    /// Intern the given value if it is not already interned. Return
    /// the arena ID for the interned value (may be an existing ID if the
    /// given value was already interned from a previous call).
    pub fn intern(&mut self, value: T) -> Id<T> {
        if let Some(id) = self.id_mapping.get(&value) {
            *id
        } else {
            let id = self.arena.alloc(value.clone());
            self.id_mapping.insert(value, id);
            id
        }
    }

    /// Get the interned data defined by the given ID. If the ID does not
    /// correspond to an interned item, then this will panic (but this should
    /// not ever be possible!)
    pub fn get(&self, id: Id<T>) -> &T {
        &self.arena[id]
    }
}
