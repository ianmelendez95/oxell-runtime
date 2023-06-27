mod prog;
mod builtins;

use prog::prog;
use builtins::State;

fn main() {
  let mut state: State = State::new();
  prog(&mut state);
  state.print_top();
}
