use crate::builtins::*;
use std::fmt;
use std::fmt::Formatter;
use std::fmt::Debug;
use std::ops::Deref;
use std::mem;

pub struct GcAlloc {
    nodes: Vec<*mut GcObj<Node>>
}

pub struct Gc<T> {
    ptr: *mut GcObj<T>
}

impl<T> Gc<T> {
    pub fn is_marked(&self) -> bool {
        unsafe { 
            (*self.ptr).marked
        }
    }

    pub fn mark(&mut self) {
        unsafe {
            (*self.ptr).marked = true;
        }
    }
}

pub struct GcObj<T> {
    marked: bool,
    value: T
}

impl<T: Debug> fmt::Debug for Gc<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.as_ref())
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
        unsafe { &(*self.ptr).value }
    }
}

impl fmt::Display for Gc<Node> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl<T> AsRef<T> for Gc<T> {
    fn as_ref(&self) -> &T {
        unsafe { &(*self.ptr).value }
    }
}

impl<T> AsMut<T> for Gc<T> {
    fn as_mut(&mut self) -> &mut T {
        unsafe { &mut (*self.ptr).value }
    }
}

pub type Worklist = Vec<Gc<Node>>;

impl GcAlloc {
    pub fn new() -> Self {
        GcAlloc { nodes: Vec::new() }
    }

    pub fn alloc_node(&mut self, item: Node) -> Gc<Node> {
        println!("Allocating: {:?}", &item);
        let node_ref = Box::into_raw(Box::new(GcObj { marked: false, value: item }));
        self.nodes.push(node_ref);
        Gc { ptr: node_ref }
    }

    pub fn collect(&mut self, mut worklist: Worklist) {
        Self::mark(&mut worklist);
        self.sweep();
    }

    fn mark(worklist: &mut Worklist) {
        while let Some(node) = worklist.pop() {
            node.mark_refs(worklist);
        }
    }

    pub fn sweep(&mut self) {
        unsafe {
            let mut new_nodes: Vec<*mut GcObj<Node>> = Vec::new();
            while let Some(gc_ref) = self.nodes.pop() {
                if (&*gc_ref).marked {
                    (&mut *gc_ref).marked = false;
                    new_nodes.push(gc_ref);
                } else {
                    println!("Sweeping");
                    let _ = Box::from_raw(gc_ref); // let it get collected when exiting scope
                }
            }
            self.nodes = new_nodes;
        }
    }

    pub fn dump(&self) {
        println!("\n--- Begin GC Stats ---\n");
        println!("Objects: {} ({} bytes)", self.nodes.len(), self.nodes.len() * mem::size_of::<Node>());
        println!("\n--- End GC Stats ---\n");
    }
}
