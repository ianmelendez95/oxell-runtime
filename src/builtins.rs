use std::fmt;
use std::fmt::Formatter;
use std::rc::Rc;

#[derive(Clone)]
pub enum Node {
    Int(i64),
}

pub type Stack = Vec<Rc<Node>>;

pub struct State {
    pub stack: Stack,
}

impl State {
    pub fn new() -> State {
        return State {
            stack: Vec::new()
        };
    }

    pub fn push_int(self: &mut State, int_val: i64) {
        self.stack.push(Rc::from(Node::Int(int_val)));
    }

    pub fn print_top(self: &State) {
        let x: &Rc<Node> = self.stack.get(self.stack.len() - 1).unwrap();
        println!("{}", x);
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Node::Int(i) => {
                write!(f, "{}", i)
            }
        }
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Node::Int(i) => {
                write!(f, "Int({})", i)
            }
        }
    }
}
