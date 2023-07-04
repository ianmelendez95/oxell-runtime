mod prog;
mod builtins;

use prog::prog;

fn main() {
  println!("{}", prog());
}
