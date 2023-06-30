use crate::builtins;
use crate::builtins::State;

pub fn prog(state: &mut State) {
    builtins::push_int(state, 13);  
    builtins::push_int(state, 2);   
    builtins::push_int(state, 42);  
    builtins::push_int(state, 9);   
    builtins::mul(state);         
    builtins::div(state);
    builtins::push_int(state, 23);
    builtins::add(state);
    builtins::sub(state);
}