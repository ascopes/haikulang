use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

/// Logical representation of variable scoping within a lowered intermediate representation.
///
/// Allows pushing and popping variable frames in the logical scope.
#[derive(Clone, Debug)]
pub struct SymbolTable<Key: Clone + Eq + Hash, Value> {
    stack: Vec<HashMap<Key, Value>>,
}

impl<Key: Clone + Debug + Eq + Hash, Value> SymbolTable<Key, Value> {
    pub fn new() -> Self {
        Self {
            stack: Vec::with_capacity(0),
        }
    }

    pub fn push(&mut self) {
        self.stack.push(HashMap::new());
    }

    pub fn pop(&mut self) {
        self.stack.pop().expect("stack underflow");
    }

    pub fn declare(&mut self, key: Key, value: Value) {
        let last_frame = self.stack.last_mut().expect("stack underflow");

        // TODO(ascopes): handle already-defined-symbols as a Result type.
        // Once try_insert is stable, we can use that instead of doing two operations here
        // (i.e. contains_key followed by insert).
        if last_frame.contains_key(&key) {
            panic!("Variable {:?} was already defined", key)
        } else {
            last_frame.insert(key, value);
        }
    }

    pub fn lookup(&self, key: &Key) -> Option<&Value> {
        self.stack
            .iter()
            .rev()
            .map(|m| m.get(key))
            .skip_while(Option::is_none)
            .next()
            .flatten()
    }
}
