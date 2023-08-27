use futures::executor::block_on;

async fn hello_world() {
    println!("hello, world!");
}

async fn sub(left: usize, right: usize) -> usize {
    left - right
}


fn main() {
    let future = hello_world(); // Nothing is printed
    block_on(future); // `future` is run and "hello, world!" is printed
    let result = block_on(sub(4, 2));
    println!("result: {}", result);
}