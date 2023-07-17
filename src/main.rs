mod prog;
mod builtins;
mod gc;

use prog::prog;
use builtins::State;

fn main() {
    let mut state = State::new();
    prog(&mut state);
    state.eval();
    // state.unwind();
    println!("{:?}", state.stack_pop());
}
