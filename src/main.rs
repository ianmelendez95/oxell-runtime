mod prog;
mod builtins;
mod gc;

use prog::prog;
use builtins::State;

fn main() {
    let mut state = State::new();
    prog(&mut state);
    // state.unwind();
    println!("{:?}", state.stack.get(state.stack.len() - 1));
}
