use std::fmt;
use std::fmt::Formatter;
use std::rc::Rc;
use std::cell::RefCell;
use std::cell::RefMut;
use crate::gc::*;

pub struct State {
    alloc: GcAlloc,
    pub stacks: Vec<Stack>
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

    pub fn alloc_node(&mut self, node: Node) -> Gc<Node> {
        self.collect();
        self.alloc.alloc_node(node)
    }

    fn collect(&mut self) {
        self.mark_from_roots();
        self.alloc.sweep();
    }
    
    fn mark_from_roots(&mut self) {
        let mut worklist: Vec<Gc<Node>> = Vec::new();

        for stack in self.stacks.iter_mut() {
            for node in stack.iter_mut() {
                match *node {
                    Node::Int(_) => {},
                    Node::FnDef(_) => {},
                    Node::App(mut nl, mut nr) => {
                        if !nl.is_marked() {
                            nl.mark();
                            worklist.push(nl);
                            State::mark(&mut worklist);
                        }

                        if !nr.is_marked() {
                            nr.mark();
                            worklist.push(nr);
                            State::mark(&mut worklist);
                        }
                    },
                    Node::ThunkRef(_) => todo!(),
                }
            }
        }
    }

    fn mark(worklist: &mut Vec<Gc<Node>>) {
        while let Some(node) = worklist.pop() {
            match *node {
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
            }
        }
    }

    fn mark_node(worklist: &mut Vec<Gc<Node>>, node: Node) {
        match node {
            Node::Int(_) => {},
            Node::FnDef(_) => {},
            Node::App(mut nl, mut nr) => {
                if !nl.is_marked() {
                    nl.mark();
                    worklist.push(nl);
                    State::mark(worklist);
                }

                if !nr.is_marked() {
                    nr.mark();
                    worklist.push(nr);
                    State::mark(worklist);
                }
            },
            Node::ThunkRef(_) => todo!(),
        }
    }

    pub fn stack_enter_new(&mut self) {
        self.stacks.push(Vec::new());
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

    // pub fn stack_clone_top(&self) -> Node {
    //     self.get_cur_stack_mut()
    // }

    fn get_cur_stack(&self) -> &Stack {
        let len = self.stacks.len();
        &self.stacks.get(len - 1).unwrap()
    }

    fn get_cur_stack_mut(&mut self) -> &mut Stack {
        let len = self.stacks.len();
        self.stacks.get_mut(len - 1).unwrap()
    }
}

#[derive(Clone, Copy)]
pub struct FnDef {
    name: &'static str,
    arity: usize,
    fn_ref: StateFn
}

#[derive(Clone, Copy)]
pub enum Node {
    Int(i64),
    FnDef(FnDef),
    App(Gc<Node>, Gc<Node>),
    ThunkRef(Gc<Thunk>)
}

// impl Node {
//     pub fn eval(self) -> Node {
//         self.reduce();

//         match self {
//             Node::Int(_) => self,
//             Node::ThunkRef(t_ref) => {
//                 match t_ref.as_ref() {
//                     Thunk::UThunk(_) => panic!(),
//                     Thunk::EThunk(value) => {
//                         value.clone()
//                     }
//                 }
//             }
//             Node::App(_, _) => todo!(),
//             Node::FnDef(_) => todo!(),
//         }
//     }

//     fn reduce(&self) {
//         if let Node::ThunkRef(t_ref) = self {
//             *t_ref = 
//             RefMut::map(t_ref.as_ref().borrow_mut(), |t_mut| {
//                 if let Thunk::UThunk(thunk) = t_mut {
//                     *t_mut = Thunk::EThunk(thunk.eval_thunk().eval());
//                     t_mut
//                 } else {
//                     t_mut  // noop
//                 }
//             });

//             if let Thunk::UThunk(_) = &*t_ref.borrow() {
//                 self.reduce();
//             }
//         }
//     }
// }

pub enum Thunk {
    UThunk(Box<dyn ThunkEval>),
    EThunk(Node)
}

pub trait ThunkEval {
    fn eval_thunk(&self) -> Node;
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

impl State {
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

    pub fn push_int(&mut self, int_val: i64) {
        self.stack_push(Node::Int(int_val));
    }

    pub fn push_fn(&mut self, fn_def: FnDef) {
        self.stack_push(Node::FnDef(fn_def));
    }

    pub fn app(&mut self) {
        let nl = self.stack_pop();
        let nr = self.stack_pop();

        let boxed_nl = self.alloc_node(nl);
        let boxed_nr = self.alloc_node(nr);

        self.stack_push(Node::App(boxed_nl, boxed_nr));
    }

    pub fn eval(&mut self) {
        // SPJ:321

        if let Node::App(_, _) = self.stack_peek() {
            let app_head = *self.stack_peek();  // copy the node
            self.stack_enter_new();
            self.stack_push(app_head);
            self.unwind();
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

pub static FN_ADD: FnDef = FnDef {
    name: "add",
    arity: 2,
    fn_ref: eval_add
};

// pub fn thunk(boxed_t: Box<dyn ThunkEval>) -> Node {
//     Node::ThunkRef(Rc::new(RefCell::new(Thunk::UThunk(boxed_t))))
// }

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
    println!("EVAL ADD");
    bin_arith!(state, +);
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


// fn eval_add(nl: Node, nr: Node) -> Node {
//     bin_arith!(nl, nr, +);
// }

// fn eval_sub(nl: Node, nr: Node) -> Node {
//     bin_arith!(nl, nr, -);
// }

// fn eval_div(nl: Node, nr: Node) -> Node {
//     bin_arith!(nl, nr, /);
// }

// fn eval_mul(nl: Node, nr: Node) -> Node {
//     bin_arith!(nl, nr, *);
// }