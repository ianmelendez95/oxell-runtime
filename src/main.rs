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
    // state.stack_dump();
    state.eval();
    // state.stack_dump();
    println!("{:?}", state.stack_pop());
}
