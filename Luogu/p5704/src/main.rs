use std::io;

fn main() {
    let mut chr = String::new();

    io::stdin()
        .read_line(&mut chr)
        .expect("Fail to read line");

    let chr = chr.trim();

    for i in chr.chars() {
        println!("{}", i.to_uppercase());
    }
}
