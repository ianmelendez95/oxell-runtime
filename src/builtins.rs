use std::fmt;
use std::fmt::Formatter;
use crate::gc::*;

pub struct State {
    alloc: GcAlloc,
    pub stacks: Vec<Stack>
}

#[derive(Clone, Copy)]
pub struct FnDef {
    pub name: &'static str,
    pub arity: usize,
    pub fn_ref: StateFn
}

#[derive(Clone, Copy)]
pub enum Node {
    Int(i64),
    FnDef(FnDef),
    App(Gc<Node>, Gc<Node>),
    ThunkRef(Gc<Thunk>),
    NodeRef(Gc<Node>)
}

pub enum Thunk {
    UThunk(Box<dyn ThunkEval>),
    EThunk(Node)
}

pub trait ThunkEval {
    fn eval_thunk(&self) -> Node;
}

type Stack = Vec<Node>;

pub type StateFn = fn(state: &mut State);

impl State {
    pub fn new() -> Self {
        let mut state = State {
            alloc: GcAlloc::new(),
            stacks: Vec::new()
        };
        state.stack_enter_new();
        state
    }

    /* ***** *
     * Debug *
     * ***** */

    pub fn gc_dump(&self) {
        self.alloc.dump();
    }

    pub fn stack_dump(&self) {
        println!("--- BEGIN DUMP ---");
        for (stack_idx, stack) in self.stacks.iter().enumerate().rev() {
            println!("\n--- Frame [{}] ---", stack_idx);
            for (node_idx, node) in stack.iter().enumerate().rev() {
                println!("[{}] {:?}", node_idx, node);
            }
        }
        println!("\n--- END DUMP ---")
    }

    /* ***** *
     * GC    *
     * ***** */

    fn alloc_node(&mut self, node: Node) -> Gc<Node> {
        let mut worklist = Vec::new();
        node.mark_refs(&mut worklist);
        self.collect(worklist);

        self.alloc.alloc_node(node)
    }

    fn alloc_nodes(&mut self, node1: Node, node2: Node) -> (Gc<Node>, Gc<Node>) {
        let mut worklist = Vec::new();
        node1.mark_refs(&mut worklist);
        node2.mark_refs(&mut worklist);
        self.collect(worklist);

        let alloc1 = self.alloc.alloc_node(node1);
        let alloc2 = self.alloc.alloc_node(node2);

        (alloc1, alloc2)
    }

    pub fn collect(&mut self, mut worklist: Worklist) {
        self.mark_stack_roots(&mut worklist);
        self.alloc.collect(worklist);
    }

    fn mark_stack_roots(&mut self, worklist: &mut Vec<Gc<Node>>) {
        for stack in self.stacks.iter_mut() {
            for node in stack.iter_mut() {
                node.mark_refs(worklist);
            }
        }
    }

    /* ***** *
     * Stack *
     * ***** */

    pub fn stack_enter_new(&mut self) {
        self.stacks.push(Vec::new());
    }

    pub fn stack_exit(&mut self) {
        self.stacks.pop();
    }

    pub fn stack_push(&mut self, node: Node) {
        self.get_cur_stack_mut().push(node);
    }

    pub fn stack_pop(&mut self) -> Node {
        self.get_cur_stack_mut().pop().unwrap()
    }

    pub fn stack_size(&self) -> usize {
        self.get_cur_stack().len()
    }

    pub fn stack_peek(&self) -> &Node {
        let stack = self.get_cur_stack();
        stack.get(stack.len() - 1).unwrap()
    }

    fn get_cur_stack(&self) -> &Stack {
        let len = self.stacks.len();
        &self.stacks.get(len - 1).unwrap()
    }

    fn get_cur_stack_mut(&mut self) -> &mut Stack {
        let len = self.stacks.len();
        self.stacks.get_mut(len - 1).unwrap()
    }

    pub fn push_int(&mut self, int_val: i64) {
        self.stack_push(Node::Int(int_val));
    }

    pub fn push_fn(&mut self, fn_def: FnDef) {
        self.stack_push(Node::FnDef(fn_def));
    }

    /* ********** *
     * Statements *
     * ********** */

    pub fn mk_ap(&mut self) {
        let raw_nl = self.stack_pop();
        let raw_nr = self.stack_pop();

        let (nl, nr) = self.alloc_nodes(raw_nl, raw_nr);

        self.stack_push(Node::App(nl, nr));
    }

    pub fn eval(&mut self) {
        // SPJ:321

        match self.stack_peek() {
            Node::FnDef(_) => {
                if let Node::FnDef(fn_def) = self.stack_pop() {
                    self.stack_enter_new();
                    (fn_def.fn_ref)(self);
                    let unwind_result = self.stack_pop();
                    self.stack_exit();
                    self.stack_push(unwind_result);
                } else {
                    unreachable!();
                }
            },
            Node::App(_, _) => {
                let app_head = self.stack_pop();  // copy the node
                self.stack_enter_new();
                self.stack_push(app_head);
                self.unwind();
                let unwind_result = self.stack_pop();
                self.stack_exit();
                self.stack_push(unwind_result);
            },
            _ => {}
        }
    }

    pub fn unwind(&mut self) {
        // SPJ:322

        while let Node::App(nl, _) = self.stack_peek() {
            self.stack_push(**nl);
        }

        if let Node::FnDef(_) = self.stack_peek() {
            if let Node::FnDef(fn_def) = self.stack_pop() {
                let stack_size = self.stack_size();
                if stack_size >= fn_def.arity {
                    let new_size = stack_size - fn_def.arity;
                    let args_spine = self.get_cur_stack_mut().split_off(new_size);
                    for arg_app in args_spine {
                        if let Node::App(_, nr) = arg_app {
                            self.stack_push(*nr);
                        } else {
                            unreachable!("Should only be applications on spine");
                        }
                    }

                    (fn_def.fn_ref)(self);
                }
            } else {
                unreachable!()
            }
        }
    }
}

impl Node {
    /// The function to 'mark' any contained GC references.
    /// Sets the `marked` bit on the GC references,
    /// as well as add them to the `worklist`.
    pub fn mark_refs(&self, worklist: &mut Worklist) {
        match self {
            Node::Int(_) => {},
            Node::FnDef(_) => {},
            Node::App(mut nl, mut nr) => {
                if !nl.is_marked() {
                    nl.mark();
                    worklist.push(nl);
                }

                if !nr.is_marked() {
                    nr.mark();
                    worklist.push(nr);
                }
            },
            Node::ThunkRef(_) => todo!(),
            Node::NodeRef(mut node_ref) => {
                if !node_ref.is_marked() {
                    node_ref.mark();
                    worklist.push(node_ref);
                }
            },
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Node::Int(i) => {
                write!(f, "{}", i)
            }
            Node::ThunkRef(t) => {
                unreachable!("Asked to display thunk: {:?}", *t);
            },
            Node::App(_, _) => {
                unreachable!("Asked to display application: {:?}", self);
            },
            Node::FnDef(_) => {
                unreachable!("Asked to display function: {:?}", self);
            }
            Node::NodeRef(nref) => write!(f, "{}", nref.as_ref()),
        }
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Node::Int(i) => write!(f, "Int({})", i), 
            Node::ThunkRef(t) => write!(f, "{:?}", t.as_ref()),
            Node::App(el, er) => {
                write!(f, "@({:?}, {:?})", el, er)
            },
            Node::FnDef(def) => write!(f, "fn<{}:{}>", def.name, def.arity),
            Node::NodeRef(nref) => write!(f, "{}", nref.as_ref()),
        }
    }
}

impl fmt::Debug for Thunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Thunk::UThunk(_) => write!(f, "#[UNEVALED]"),
            Thunk::EThunk(val) => write!(f, "#[{:?}]", val),
        }
    }
}


/* ***************** *
 * Builtin Functions *
 * ***************** */

pub static FN_ADD: FnDef = FnDef {
    name: "add",
    arity: 2,
    fn_ref: eval_add
};

pub static FN_SUB: FnDef = FnDef {
    name: "sub",
    arity: 2,
    fn_ref: eval_sub
};

pub static FN_MUL: FnDef = FnDef {
    name: "mul",
    arity: 2,
    fn_ref: eval_mul
};

pub static FN_DIV: FnDef = FnDef {
    name: "div",
    arity: 2,
    fn_ref: eval_div
};

macro_rules! bin_arith {
    ($state:ident, $op:tt) => {
        $state.eval(); 
        let vl = $state.stack_pop();

        $state.eval();
        let vr = $state.stack_pop();

        if let Node::Int(vl) = vl {
            if let Node::Int(vr) = vr {
                $state.stack_push(Node::Int(vl $op vr));
            } else {
                panic!("Expecting integer for right operand: {:?}", vr)
            }
        } else {
            panic!("Expecting integer for left operand: {:?}", vl)
        }
    };
}

pub fn eval_add(state: &mut State) {
    bin_arith!(state, +);
}

pub fn eval_sub(state: &mut State) {
    bin_arith!(state, -);
}

pub fn eval_mul(state: &mut State) {
    bin_arith!(state, *);
}

pub fn eval_div(state: &mut State) {
    bin_arith!(state, /);
}

pub struct TracedThunk {
    name: String,
    value: Node
}

impl ThunkEval for TracedThunk {
    fn eval_thunk(&self) -> Node {
        println!("Evaling: {}", self.name);
        self.value.clone()
    }
}

// impl TracedThunk {
//     pub fn new(name: String, value: Node) -> Node {
//         thunk(Box::new(TracedThunk { name, value }))
//     }
// }

// macro_rules! bin_thunk {
//     ($thunk_name:ident, $eval_fn:ident, $fn_name:ident) => {
//         struct $thunk_name {
//             nl: Node,
//             nr: Node
//         }

//         impl ThunkEval for $thunk_name {
//             fn eval_thunk(&self) -> Node {
//                 $eval_fn(self.nl.clone(), self.nr.clone())
//             }
//         }

//         pub fn $fn_name(nl: Node, nr: Node) -> Node {
//             thunk(Box::new($thunk_name { nl, nr }))
//         }
//     }
// }

// bin_thunk!(AddThunk, eval_add, add);
// bin_thunk!(SubThunk, eval_sub, sub);
// bin_thunk!(DivThunk, eval_div, div);
// bin_thunk!(MulThunk, eval_mul, mul);
