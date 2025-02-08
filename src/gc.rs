use crate::builtins::*;
use std::fmt;
use std::fmt::Formatter;
use std::fmt::Debug;
use std::ops::Deref;
use std::mem;

pub struct GcState {
    nodes: Vec<*mut GcObj<dyn Mark>>
}

pub fn store_boxed<T>(value: T) -> Box<T> {
    Box::new(value)
}

pub struct Gc<T: Mark + ?Sized + 'static> {
    ptr: *mut GcObj<T>
}

pub struct GcObj<T: Mark + ?Sized + 'static> {
    marked: bool,
    value: T
}

pub trait Mark {
    fn mark_refs(&self, worklist: &mut Vec<&dyn Mark>);
}

pub type Nodelist = Vec<*mut GcObj<dyn Mark>>;

pub type Worklist<'a> = Vec<&'a dyn Mark>;

impl<T: Mark> Gc<T> {
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

impl<T: Debug + Mark> fmt::Debug for Gc<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.as_ref())
    }
}

impl<T: Mark + ?Sized + 'static> Copy for Gc<T> {}

impl<T: Mark + ?Sized + 'static> Clone for Gc<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: Mark + ?Sized> Deref for Gc<T> {
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

impl<T: Mark> AsRef<T> for Gc<T> {
    fn as_ref(&self) -> &T {
        unsafe { &(*self.ptr).value }
    }
}

impl<T: Mark> AsMut<T> for Gc<T> {
    fn as_mut(&mut self) -> &mut T {
        unsafe { &mut (*self.ptr).value }
    }
}

impl GcState {
    pub fn new() -> Self {
        GcState { nodes: Vec::new() }
    }

    pub fn alloc<T: Mark>(&mut self, value: T) -> Gc<T> {
        let value_ref = Box::into_raw(Box::new(GcObj { marked: false, value }));
        self.nodes.push(value_ref);
        Gc { ptr: value_ref }
    }

    pub fn alloc_node(&mut self, item: Node) -> Gc<Node> {
        self.alloc(item)
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
            let mut new_nodes: Nodelist = Vec::new();
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
