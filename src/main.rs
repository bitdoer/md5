use md5::lib::{convert_and_pad, md5_hash};

fn main() {
    let input = std::env::args().collect::<Vec<String>>();
    for i in md5_hash(convert_and_pad(&input[1])).iter() {
        print!("{:01$x}", i, 8);
    }
    println!();
}
