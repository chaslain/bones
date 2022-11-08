use std::collections::HashMap;

use rand::{thread_rng, Rng};

pub fn roll_dice(dice_num: i32) -> Vec<i32> {
    let mut thread = thread_rng();

    let mut result: Vec<i32> = Vec::new();

    for _ in 0..dice_num {
        result.push(thread.gen_range(1..=6));
    }

    result
}

pub fn score_dice(dice: Vec<i32>) -> (i32, i32) {
    let mut score = 0;
    let mut remaining_dice = dice.len() as i32;


    let mut quants = get_quants(dice);

    for (dice_num, quantity) in quants.iter_mut() {
        if *quantity >= 3 {
            remaining_dice -= 3;
            score += if *dice_num == 1 { 1000 } else { dice_num * 100 };
            *quantity -= 3;
        }
    }

    score += get_score_quantity(&quants, 1, 100);
    score += get_score_quantity(&quants, 5, 50);

    (score, remaining_dice)
}

fn get_score_quantity(quants: &HashMap<i32, i32>, num: i32, score: i32) -> i32 {
    match quants.get(&num) {
        Some(amount) => score * *amount,
        None => 0,
    }
}

fn get_quants(dice: Vec<i32>) -> HashMap<i32, i32> {
    let mut result: HashMap<i32, i32> = HashMap::new();

    for i in dice.iter() {
        match result.get_mut(i) {
            Some(num) => {
                *num += 1;
            }
            None => {
                result.insert(*i, 1);
            }
        }
    }

    result
}
