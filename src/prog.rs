use crate::builtins::*;

pub fn prog() -> Node {
    let x: Node = Node::ThunkRef(Box::from(Thunk::UThunk(let_x_1_5)));
    return add(x, Node::Int(3));
}

fn let_x_1_5() -> Node {
    return add(Node::Int(1), Node::Int(2));
}