use std::collections::HashMap;
use std::rc::Rc;
use std::fmt;
use std::fmt::Formatter;

pub type UBInt = i64;

#[derive(Debug,Copy,Clone)]
pub enum GCode {

}

#[derive(Clone)]
pub enum GNode {
  GInt(UBInt),
}

pub struct DNode {
  pub stack: GStack,
  pub prog: GProg
}

pub type GStack = Vec<Rc<GNode>>;
pub type GGraph = HashMap<String, GNode>;
pub type GProg = Vec<GCode>;
pub type GDump = Vec<DNode>;

pub struct State {
  pub stack: GStack,
  pub graph: GGraph,
  pub prog: GProg,
  pub dump: GDump
}

impl State {
  pub fn new() -> State {
    return State {
      stack: Vec::new(),
      graph: HashMap::new(),
      prog: Vec::new(),
      dump: Vec::new()
    };
  }

  pub fn push_int(self: &mut State, int_val: UBInt) {
    self.stack.push(Rc::from(GNode::GInt(int_val)));
  }

  pub fn print_top(self: &State) {
    let x: &Rc<GNode> = self.stack.get(self.stack.len() - 1).unwrap();
    println!("{:?}", x);
  }
}

impl fmt::Debug for GNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      GNode::GInt(i) => {
        write!(f, "Int({})", i)
      }
    }
  }
}