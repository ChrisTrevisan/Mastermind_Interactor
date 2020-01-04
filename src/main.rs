extern crate rand;
use std::io;
use rand::Rng;
fn main() {
    let mut rng = rand::thread_rng();
    let secret : String = format!("{:06}",rng.gen_range(0,999999));
    let mut scnt : [i32;10] = [0;10];
    println!("{}",secret);
    for c in secret.chars() {
        scnt[c as usize - '0' as usize] += 1;
    }
    let mut won : bool = false;
    let mut moves : i32 =  0;
    while !won {
        let mut guess : String = String::new();
        let mut correct : bool = false;
        while !correct {
            guess = String::new();
            io::stdin().read_line(&mut guess).ok().expect("Failed to read input from stdin.");
            guess = guess.trim().to_string();
            if guess.len() != 6 || !guess.chars().all(char::is_numeric) {
                println!("Please print a 6 digit guess.");
            } else {
                correct = true;
            }
        }
        let mut same : i32 = 0;
        let mut moved : i32 = 0;
        let mut cnt : [i32;10] = [0;10];
        for c in guess.chars() {
            cnt[c as usize - '0' as usize] += 1;
        }
        for it in secret.chars().zip(guess.chars()) {
            let (a,b) = it;
            if a == b {
                same += 1;
                cnt[a as usize - '0' as usize] -= 1;
                scnt[a as usize - '0' as usize] -= 1;
            }
        }
        for it in cnt.iter().zip(scnt.iter()) {
            let (a,b) = it;
            moved += std::cmp::min(a,b);
        }
        for it in secret.chars().zip(guess.chars()) {
            let (a,b) = it;
            if a == b {
                scnt[a as usize - '0' as usize] += 1;
            }
        }
        moves += 1;
        println!("{} digit{} in the right place, {} digit{} in the wrong place.",same,if same==1{" is"}else{"s are"},moved,if moved==1{" is"}else{"s are"});
        if same == 6 {
            won = true;
        }
    }
    println!("Congratulations! You won in {} moves!",moves);
}