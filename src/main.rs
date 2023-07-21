mod prog;
mod builtins;
mod gc;

use prog::eval_prog;
use builtins::State;

use std::{rc::Rc, cell::RefCell};
use std::fmt::Display;

use crate::prog::*;

fn main() {
    let mut state = State::new();
    state.push_fn(FN_PROG);
    state.eval();
    println!("{}", state.stack_pop());
    // state.collect(Vec::new());
    // state.gc_dump();
    // state.gc_dump();
}
