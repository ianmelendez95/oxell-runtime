// use crate::builtins::*;

// struct T0 {
//     x: Node
// }

// impl ThunkEval for T0 {
//     fn eval_thunk(&self) -> Node {
//         let x: Node = self.x.clone();
//         add(int(3), x)
//     }
// }

// pub fn prog() -> Node {
//     let x: Node = add(int(1), int(2));
//     add(thunk(Box::from(T0 { x: x.clone() })), x)
// }

use crate::builtins::*;

pub static FN_PROG: FnDef = FnDef {
    name: "prog",
    arity: 2,
    fn_ref: eval_prog
};

pub fn eval_prog(state: &mut State) {
    state.push_int(13);

    state.push_int(2);
    
    state.push_int(42);
    state.push_int(9);
    state.push_fn(FN_MUL);
    state.mk_ap();
    state.mk_ap();

    state.push_fn(FN_DIV);
    state.mk_ap();
    state.mk_ap();

    state.push_int(23);
    state.push_fn(FN_ADD);
    state.mk_ap();
    state.mk_ap();

    state.push_fn(FN_SUB);
    state.mk_ap();
    state.mk_ap();

    state.unwind();
}