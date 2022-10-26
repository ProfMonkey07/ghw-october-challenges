use std::io;

fn main() {
    let mut toReverse = String::new();
    let mut reversed = String::new();
    print!("input string to reverse \n");
    io::stdin().read_line(&mut toReverse).expect("segfault lmaoao");
    for i in 0..toReverse.len() {
        let toAdd = (toReverse.as_bytes()[toReverse.len() - i - 1] as char).to_string();
        reversed += &toAdd;
    }
    print!("{} \n", reversed);

}
