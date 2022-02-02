use my_wordcount::count;
use std::env;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let filename = env::args().nth(1).expect("1 argument FILENAME required");
    //let filename = "text.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(&file);

    let freqs = count(reader, Default::default());
    println!("{:?}", freqs);
}
