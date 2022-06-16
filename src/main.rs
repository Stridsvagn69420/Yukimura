use yukimura::float32::abc;

fn main() {
    println!("Hello, world!");
    match abc::pq(6.0, 5.0 ) {
        Ok(s) => println!("{:?} {:?}", s.x1, s.x2),
        Err(e) => println!("{}", e)
    }
}
