use watchexec::run::watch;

mod watch;

fn main() {
    println!("Hello World!");

    let my_handler = watch::DevHandler {};
    let w = watch(&my_handler);
    let w = match w {
        Ok(_f) => "Success",
        Err(error) => panic!("Error watching: {:?}", error),
    };
    println!("{:?}", w);
}
