use crate::builtins as bi;
use crate::builtins::State;

pub fn prog(state: &mut State) {
    bi::int(state, 13);  
    bi::int(state, 2);   
    bi::int(state, 42);  
    bi::int(state, 9);   
    bi::mul(state);         
    bi::div(state);
    bi::int(state, 23);
    bi::add(state);
    bi::sub(state);
}