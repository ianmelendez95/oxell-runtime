use crate::builtins as bi;
use crate::builtins::State;

pub fn prog(state: &mut State) {
    bi::int(state, 3);
    bi::thunk(state, let_x_1_5);
    bi::add(state);
}

fn let_x_1_5(state: &mut State) {
    bi::int(state, 2);
    bi::int(state, 1);
    bi::add(state);
}