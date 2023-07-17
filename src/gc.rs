use crate::builtins::*;
use std::fmt;
use std::fmt::Formatter;
use std::fmt::Debug;
use std::ops::Deref;

pub type Stack = Vec<Node>;

pub struct GcAlloc {
    nodes: Vec<*mut Node>
}

pub struct Gc<T> {
    ptr: *mut T
}

impl<T: Debug> fmt::Debug for Gc<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<T> Copy for Gc<T> {}

impl<T> Clone for Gc<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Deref for Gc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr }
    }
}

impl fmt::Display for Gc<Node> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl<T> AsRef<T> for Gc<T> {
    fn as_ref(&self) -> &T {
        unsafe { &*self.ptr }
    }
}

impl GcAlloc {
    pub fn new() -> Self {
        GcAlloc { nodes: Vec::new() }
    }

    pub fn new_node(&mut self, item: Node) -> Gc<Node> {
        unsafe {
            let node_ref = Box::into_raw(Box::new(item));
            self.nodes.push(node_ref);
            Gc { ptr: node_ref }
        }
    }
}
