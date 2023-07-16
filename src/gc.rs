use crate::builtins::*;

pub struct Gc<T> {
    ptr: *mut T
}

pub type Stack = Vec<Node>;

