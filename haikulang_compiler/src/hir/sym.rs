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
        Self { stack: Vec::new() }
    }

    /// Push a new frame for variable scoping, creating a new variable scope.
    pub fn push(&mut self) {
        self.stack.push(HashMap::new());
    }

    /// Pop an existing frame for variable scoping, effectively leaving the current
    /// variable scope.
    ///
    /// This will panic if no scopes are left on the stack.
    pub fn pop(&mut self) {
        self.stack.pop().expect("stack underflow");
    }

    /// Declare an item in the current scope.
    ///
    /// This panics if no scope exists, or if the variable was already defined in
    /// the current scope. We do however allow shadowing outer scopes.
    pub fn declare(&mut self, key: Key, value: Value) {
        let last_frame = self.stack.last_mut().expect("no frame in scope");

        // TODO(ascopes): handle already-defined-symbols as a Result type rather than
        // panicking. We can emit Error types in the HIR nodes to describe these issues
        // and report them to an error collector.

        // Once try_insert is stable, we can use that instead of doing two operations here
        // (i.e. contains_key followed by insert).
        if last_frame.contains_key(&key) {
            panic!("Variable {:?} was already defined", key)
        } else {
            last_frame.insert(key, value);
        }
    }

    /// Attempt to fetch the definition for the current variable. This checks each
    /// scope in reverse from the top of the stack to the bottom, returning None if
    /// no definition was found.
    pub fn lookup(&self, key: &Key) -> Option<&Value> {
        self.stack
            .iter()
            // Start at the top of the stack and work our way down.
            .rev()
            // Fetch the variable from each scope.
            .map(|m| m.get(key))
            // Continue until we find the first non-None match, and then stop.
            .skip_while(Option::is_none)
            .next()
            .flatten()
    }
}
