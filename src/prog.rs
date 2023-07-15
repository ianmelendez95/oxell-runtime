use crate::builtins::*;

struct T0 {
    x: Node
}

impl ThunkEval for T0 {
    fn eval_thunk(&self) -> Node {
        let x: Node = self.x.clone();
        add(int(3), x)
    }
}

pub fn prog() -> Node {
    let x: Node = add(int(1), int(2));
    add(thunk(Box::from(T0 { x: x.clone() })), x)
}