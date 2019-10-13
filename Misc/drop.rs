// std::mem::drop 
 
// {
  // use `Pin::new_unchecked(&mut foo)`
//  drop(foo);
}

#[derive(Debug)]
struct A;

fn main() -> () {
    let a = A;
    std::mem::drop(a);
    println!("{:?}", a);
}
