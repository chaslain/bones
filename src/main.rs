use std::{collections::HashMap, fs::File, io::Write, ops::Add, sync::Arc, thread::{self, JoinHandle}, env};

mod game_logic;

const MIN_SCORE: i32 = 500;
const WINNING_SCORE: i32 = 10000;
const NUM_OF_GAMES: i32 = 50000;

fn main() {
    let mut aggression_list: Vec<i32> = Vec::new();
    
    let args: Vec<String> = env::args().collect();

    let num_threads = args.get(01).unwrap().parse::<i32>().unwrap();
    let num_games = args.get(2).unwrap().parse::<i32>().unwrap();
    
    

    for i in 0..50 {
        aggression_list.push(i * 10 + 90);
    }

    let arc: Arc<Vec<i32>> = Arc::new(aggression_list);

    let mut thread_handles: Vec<JoinHandle<Master>> = Vec::new();

    for i in 0..num_threads {
        println!("starting thread with {} games", num_games);
        thread_handles.push(thread::spawn(execute_game(num_games, arc.clone(), i as i32)));
        
    }

    let mut totals = Master {
        aggression_to_success: HashMap::new()
    };

    for i in thread_handles {
        totals = totals + i.join().unwrap();
    }
    

    _ = totals.save_file();
}

fn execute_game(range: i32, aggression_list: Arc<Vec<i32>>, thread_num: i32) -> impl Fn() -> Master {
    move || -> Master {
        let mut master = Master {
            aggression_to_success: HashMap::new(),
        };


        for i in 0..range {
            let mut game = Game::new_game(&*aggression_list);

            game.play();
            let index = game.winner.unwrap();
            let player = game.players.get(index as usize).unwrap();
            master.record_game(player.aggression);

            if i % 1000 == 0
            {

                let mut f = File::create(format!("./{}", thread_num)).unwrap();
                _ = f.write(i.to_string().as_bytes());
            }
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

            if running_total + score >= WINNING_SCORE {
                break;
            }

            if is_final {
                continue;
            }

            if running_total >= self.aggression * dice_amount && self.score + running_total >= MIN_SCORE {
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
