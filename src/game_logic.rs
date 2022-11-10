
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
    let mut score: i32 = 0;
    let mut remaining_dice = dice.len() as i32;

    let mut quants = get_quants(&dice);

    for (dice_num, quantity) in quants.iter_mut().enumerate() {
        if *quantity >= 3 {
            remaining_dice -= 3;
            score += if dice_num == 1 { 1000 } else { (dice_num * 100) as i32 };
            *quantity -= 3;
        }
    }

    let (one_score, dice_consumed) = get_score_quantity(&quants, 1, 100);
    remaining_dice -= dice_consumed;
    score += one_score;
    let (one_score, dice_consumed) = get_score_quantity(&quants, 5, 50);
    remaining_dice -= dice_consumed;
    score += one_score;

    (score, remaining_dice)
}

fn get_score_quantity(quants: &[i32; 7], num: i32, score: i32) -> (i32, i32) {
    (score * quants[num as usize], quants[num as usize])
}


fn get_quants(dice: &[i32]) -> [i32; 7] {
    let mut result = [0; 7];
    dice.iter().for_each(|i| result[*i as usize] += 1);
    result
}