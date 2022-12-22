std::env;

fn main() {
    println!("This is Mini-grep!");

    let args: Vec<String> = env::args().collect();
    dbg!(args)
}
