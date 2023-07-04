use crate::builtins::*;

pub fn prog() -> Node {
    let x: Node = thunk(let_x_1_5);
    return add(x, int(3));
}

fn let_x_1_5() -> Node {
    return add(int(1), int(2));
}