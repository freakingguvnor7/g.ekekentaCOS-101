use std::io::write;

fn main() {
   let mut file = std::fs::file::open("welcome_message.txt").unwrap();
   let mut contents = Striing::new();
   file.read_to_string(&mut contents).unwrap();

    print!("{}", contents);
}
