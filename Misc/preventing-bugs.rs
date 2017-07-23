use std::rc::Rc;
use std::thread;

fn main() {
  let value = Rc::new(1);
  let shared_value = value.clone();
  let handle = thread::spawn(move || {
    println!("{}", shared_value);
  });
  handle.join().unwrap();
}
