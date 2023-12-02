use std::io;

fn main() {
    let mut line = String::new();

    io::stdin()
        .read_line(&mut line)
        .expect("Fail to read line");

    let line = line.trim();

    let mut val = [0; 2];
    for (index, words) in line.split_whitespace().enumerate() {
        val[index] = words.parse().expect("Not an integer");
    }

    println!("{}", val[0] + val[1]);
}
