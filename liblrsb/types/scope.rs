// This program is subject to the terms of the GNU General Public
// License, version 2.0. If a copy of the GPL was not distributed with
// this program, You can obtain one at http://gnu.org.

use std::hashmap::{HashMap, Entry};
use std::fmt::{Debug, Write};

use interner::{Interned};

/// A scope for identifier resolution.
///
/// = Remarks
///
/// Inner scopes can shadow outer scopes. Hence, whenever we substitute identifiers, we
/// have a set of bound identifiers and a set of unbound identifiers. E.g.,
///
///     let
///         x = 1;
///         y = 2;
///     in
///         x + let
///             x = 3;
///         in
///             x + y
///
/// will be resolved to
///
///     1 + let
///         x = 3;
///     in
///         x + 2
///
/// When we set up the substitutions for the outer `let`, we add `x` and `y` to the set of
/// bound identifiers. When we encounter the inner `let`, we add `x` to the set of unbound
/// identifiers so that we don't accidentally substitute the outer `x` inside the inner
/// `let`'s body.
pub struct Scope<T: Clone> {
    names: HashMap<Interned, Vec<Option<T>>>,
}

impl<T: Clone> Scope<T> {
    /// Creates a new scope.
    pub fn new() -> Scope<T> {
        Scope {
            names: HashMap::new().unwrap()
        }
    }

    /// Pushes an identifier and its substitution onto the scope.
    ///
    /// = Remarks
    ///
    /// Every identifier can be pushed multiple times. The later values shadow the earlier
    /// values.
    pub fn bind(&mut self, name: Interned, val: T) {
        match self.names.entry(&name).unwrap() {
            Entry::Occupied(mut o) => o.push(Some(val)),
            Entry::Vacant(v) => { v.set(name, vec!(Some(val))); },
        }
    }

    pub fn hide(&mut self, name: Interned) {
        match self.names.entry(&name).unwrap() {
            Entry::Occupied(mut o) => o.push(None),
            Entry::Vacant(v) => { v.set(name, vec!(None)); },
        }
    }

    /// Pops an identifier from the scope.
    pub fn pop(&mut self, name: Interned) {
        match self.names.entry(&name).unwrap() {
            Entry::Occupied(mut o) => {
                o.pop().unwrap();
                if o.len() == 0 {
                    o.remove();
                }
            },
            _ => abort!(),
        }
    }

    /// Returns the value associated with an identifier at the innermost scope, if any.
    pub fn get(&self, name: Interned) -> Option<T> {
        match self.names.get(&name) {
            Some(v) => v.last().map(|t| t.clone()).unwrap(),
            _ => None,
        }
    }
}

impl<T: Clone+Debug> Debug for Scope<T> {
    fn fmt<W: Write>(&self, mut w: &mut W) -> Result {
        write!(w, "Scope {{ names: {:?} }}", self.names)
    }
}
