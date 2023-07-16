use crate::builtins::*;
use std::ptr::NonNull;
use std::fmt;
use std::fmt::Formatter;

pub type Stack = Vec<Node>;

pub struct GcAlloc {
    nodes: Vec<NonNull<Node>>
}

#[derive(Clone, Copy)]
pub struct Gc<T> {
    ptr: NonNull<T>
}

impl fmt::Display for Gc<Node> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl fmt::Debug for Gc<Node> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.as_ref())
    }
}

impl<T> Gc<T> {
    pub fn as_ref(&self) -> &T {
        unsafe {
            self.ptr.as_ref()
        }
    }
}

impl GcAlloc {
    pub fn new() -> Self {
        GcAlloc { nodes: Vec::new() }
    }

    pub fn new_node(&mut self, item: Node) -> Gc<Node> {
        unsafe {
            let node_ref = NonNull::new_unchecked(Box::into_raw(Box::new(item)));
            self.nodes.push(node_ref);
            Gc { ptr: node_ref }
        }
    }
}
