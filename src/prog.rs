use crate::builtins::State;

pub fn prog(state: &mut State) {
    state.push_int(13);  // 13
    state.push_int(2);   // 2 13
    state.push_int(42);  // 42 2 13
    state.push_int(9);   // 9 42 2 13
    state.mul();                 // 378 2 13
    state.div();
    state.push_int(23);
    state.add();
    state.sub();
}