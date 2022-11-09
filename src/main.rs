use std::{collections::HashMap, fs::File, io::Write};

mod game_logic;

const MIN_SCORE: i32 = 500;
const WINNING_SCORE: i32 = 5000;

fn main() {
    
    let mut aggression_list: Vec<i32> = Vec::new();

    for i in 1..=50 {
        aggression_list.push(i * 50);
    }


    let mut master = Master {
        aggression_to_success: HashMap::new(),
    };

    for _ in 0..100000000 {
        let mut game = Game::new_game(&aggression_list);

        game.play();

        let index = game.winner.unwrap();
        let player = game.players.get(index as usize).unwrap();
        master.record_game(player.aggression);
    }

    let _ = master.save_file();
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
