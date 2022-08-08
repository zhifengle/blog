#![allow(unused_imports)]
#![allow(dead_code)]

use rand::{prelude::SliceRandom, thread_rng};

enum Colors {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}
pub fn look_poker(name: &str, cards: &Vec<&String>) {
    println!("Cards of {}", name);
    for &c in cards {
        print!("{} ", c);
    }
    println!("");
}

pub fn poker() {
    let colors = vec!["♦", "♣", "♥", "♠"];
    let numbers = vec![
        "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A",
    ];
    let mut arr = Vec::new();
    for c in colors {
        for &num in &numbers {
            // arr.push(format!("{}{}", c, num));
            let mut s = c.to_owned();
            s.push_str(num);
            arr.push(s);
        }
    }
    arr.push("Big-Joker".to_string());
    arr.push("Little-Joker".to_string());

    // @TODO shuffle arr
    // https://programming-idioms.org/idiom/10/shuffle-a-list/410/rust
    let mut rng = thread_rng();
    arr.shuffle(&mut rng);

    let mut player1 = Vec::new();
    let mut player2 = Vec::new();
    let mut player3 = Vec::new();
    let mut dp = Vec::new();

    for (i, card) in arr.iter().enumerate() {
        if i >= arr.len() - 3 {
            dp.push(card);
        } else if i % 3 == 0 {
            player1.push(card)
        } else if i % 3 == 1 {
            player2.push(card)
        } else if i % 3 == 2 {
            player3.push(card)
        }
    }
    look_poker("player1", &player1);
    look_poker("player2", &player2);
    look_poker("player3", &player3);
    look_poker("dp", &dp);
}

#[test]
fn test() {
    poker();
}
