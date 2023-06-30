use crate::builtins::State;

pub fn prog(state: &mut State) {
  state.push_int(2);
  state.push_int(1);
  state.sum();
}