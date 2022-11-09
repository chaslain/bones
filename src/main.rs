use std::{
    collections::HashMap,
    fs::File,
    io::Write,
    ops::{Add, Range},
    sync::Arc,
    thread,
};

mod game_logic;

const MIN_SCORE: i32 = 500;
const WINNING_SCORE: i32 = 5000;
const NUM_OF_GAMES: i32 = 10000000;

fn main() {
    let mut aggression_list: Vec<i32> = Vec::new();

    for i in 1..=50 {
        aggression_list.push(i * 50);
    }

    let arc: Arc<Vec<i32>> = Arc::new(aggression_list);

    let a = thread::spawn(execute_game(NUM_OF_GAMES, arc.clone()));
    let b = thread::spawn(execute_game(NUM_OF_GAMES, arc.clone()));
    let c = thread::spawn(execute_game(NUM_OF_GAMES, arc.clone()));
    let d = thread::spawn(execute_game(NUM_OF_GAMES, arc.clone()));
    let e = thread::spawn(execute_game(NUM_OF_GAMES, arc.clone()));
    let f = thread::spawn(execute_game(NUM_OF_GAMES, arc.clone()));
    let g = thread::spawn(execute_game(NUM_OF_GAMES, arc.clone()));
    let h = thread::spawn(execute_game(NUM_OF_GAMES, arc.clone()));
    let i = thread::spawn(execute_game(NUM_OF_GAMES, arc.clone()));
    let j = thread::spawn(execute_game(NUM_OF_GAMES, arc.clone()));

    let master_a: Master = a.join().unwrap();
    let master_b: Master = b.join().unwrap();
    let master_c: Master = c.join().unwrap();
    let master_d: Master = d.join().unwrap();
    let master_e: Master = e.join().unwrap();
    let master_f: Master = f.join().unwrap();
    let master_g: Master = g.join().unwrap();
    let master_h: Master = h.join().unwrap();
    let master_i: Master = i.join().unwrap();
    let master_j: Master = j.join().unwrap();

    let totals = master_a
        + master_b
        + master_c
        + master_d
        + master_e
        + master_f
        + master_g
        + master_h
        + master_i
        + master_j;

    totals.save_file();
}

fn execute_game(range: i32, aggression_list: Arc<Vec<i32>>) -> impl Fn() -> Master {
    move || -> Master {
        let mut master = Master {
            aggression_to_success: HashMap::new(),
        };

        for _ in 0..range {
            let mut game = Game::new_game(&*aggression_list);

            game.play();

            let index = game.winner.unwrap();
            let player = game.players.get(index as usize).unwrap();
            master.record_game(player.aggression);
        }

        master
    }
}

impl Game {
    pub fn new_game(aggressions: &Vec<i32>) -> Game {
        let mut players: Vec<Player> = Vec::new();

        for i in aggressions {
            players.push(Player::new(*i));
        }

        Game {
            players,
            winner: None,
        }
    }

    pub fn play(&mut self) {
        let mut is_final = false;
        let mut lead_player_id: Option<i32> = None;

        'game: loop {
            for i in 0..self.players.len() as i32 {
                match self.players.get_mut(i as usize) {
                    Some(player) => {
                        player.play_turn(is_final);

                        if player.score > WINNING_SCORE {
                            match lead_player_id {
                                Some(id) => {
                                    if id == i {
                                        // decide a winner.
                                        self.winner = Some(i);
                                        break 'game;
                                    }
                                }
                                None => {
                                    lead_player_id = Some(i);
                                }
                            }
                            is_final = true;
                        }
                    }
                    None => panic!("Unspecified Player error"),
                }
            }
        }
    }
}

impl Player {
    pub fn new(aggression: i32) -> Player {
        Player {
            aggression,
            score: 0,
        }
    }

    pub fn play_turn(&mut self, is_final: bool) {
        let mut dice_amount = 5;
        let mut running_total = 0;
        loop {
            let dice = game_logic::roll_dice(dice_amount);
            let (score, dic) = game_logic::score_dice(dice);
            dice_amount = dic;

            if score == 0 {
                running_total = 0;
                break;
            }

            if dice_amount == 0 {
                dice_amount = 5;
            }

            running_total += score;

            if running_total + score >= 5000 {
                break;
            }

            if is_final {
                continue;
            }

            if running_total >= self.aggression && self.score + running_total >= MIN_SCORE {
                break;
            }
        }

        self.score += running_total;
    }
}

impl Master {
    pub fn record_game(&mut self, winner_aggression: i32) {
        match self.aggression_to_success.get_mut(&winner_aggression) {
            Some(aggression) => {
                *aggression += 1;
            }
            None => {
                self.aggression_to_success.insert(winner_aggression, 1);
            }
        }
    }

    pub fn save_file(&self) -> std::io::Result<()> {
        let mut contents = String::from("");
        for (aggression, wins) in self.aggression_to_success.iter() {
            contents.push_str(&format!("{},{}\n", aggression, wins).to_owned());
        }

        let mut file = File::create("result.csv").unwrap();
        file.write_all(contents.as_bytes())
    }
}

impl Add<Master> for Master {
    type Output = Master;
    fn add(mut self, rhs: Master) -> Self::Output {
        for (aggression, success) in rhs.aggression_to_success {
            match self.aggression_to_success.get(&aggression) {
                Some(val) => {
                    self.aggression_to_success.insert(aggression, val + success);
                }
                None => {
                    self.aggression_to_success.insert(aggression, success);
                }
            }
        }

        self
    }
}

struct Master {
    aggression_to_success: HashMap<i32, i32>,
}

struct Game {
    players: Vec<Player>,
    winner: Option<i32>,
}

struct Player {
    score: i32,
    aggression: i32,
}
