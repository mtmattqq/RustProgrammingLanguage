use std::io;

fn main() {
    let mut chr = String::new();
    
    io::stdin()
        .read_line(&mut chr)
        .expect("Fail to read line");

    let chr = chr.trim();

    for i in 0..3 {
        for _j in 0..(3 - i - 1) {
            print!(" ");
        }
        
        for _j in 0..(i * 2 + 1) {
            print!("{}", chr);
        }
        println!();
    }
}
