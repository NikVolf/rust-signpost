use rust_signpost::*;

fn main() {

    declare_signpost(0x100);
    println!("declared signpost!");

    std::thread::park_timeout(std::time::Duration::from_secs(10));

    start_signpost(0x100);
    println!("started signpost!");

    std::thread::park_timeout(std::time::Duration::from_secs(10));

    end_signpost(0x100);
    println!("ended signpost!");

    std::thread::park_timeout(std::time::Duration::from_secs(10));
    println!("adios!");

}