mod prog;
mod builtins;
mod gc;

use prog::prog;
use builtins::State;

use std::{rc::Rc, cell::RefCell};
use std::fmt::Display;

#[derive(Debug)]
struct CantCopy { x: u32 }

fn main() {
    let mut state = State::new();
    prog(&mut state);
    state.eval();
    println!("{}", state.stack_pop());
    // state.collect(Vec::new());
    // state.gc_dump();
    // state.gc_dump();
}
