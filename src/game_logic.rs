use std::time::SystemTime;

use rand::{RngCore, SeedableRng};
use rand_xoshiro::Xoshiro256StarStar;

pub struct Rng {
    gen: Xoshiro256StarStar,
}

impl Rng {
    pub fn roll_dice(&mut self, dice_num: i32) -> [Option<i32>; 5] {
        let mut result = [None; 5];
        for i in 0..dice_num {
            let num = (self.gen.next_u32() % 6) + 1;
            result[i as usize] = Some(num as i32);
        }

        result
    }

    pub fn new() -> Rng {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Time went backwards");

        Rng {
            gen: Xoshiro256StarStar::seed_from_u64(since_the_epoch.as_millis() as u64),
        }
    }
}

pub fn score_dice(dice: &[Option<i32>; 5]) -> (i32, i32) {
    let mut score: i32 = 0;
    let mut remaining_dice = dice.len() as i32;

    let mut quants = get_quants(&dice);

    for (dice_num, quantity) in quants.iter_mut().enumerate() {
        if *quantity >= 3 {
            remaining_dice -= 3;
            score += if dice_num == 1 {
                1000
            } else {
                (dice_num * 100) as i32
            };
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

fn get_quants(dice: &[Option<i32>; 5]) -> [i32; 7] {
    let mut result = [0; 7];
    dice.iter().for_each(|i| match i {
        Some(num) => result[*num as usize] += 1,
        None => {}
    });
    result
}
