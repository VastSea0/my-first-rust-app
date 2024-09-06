mod level;
mod player;

use log::{error, info};
use std::io::stdin;
use std::process::exit;
use crate::level::Level;
use crate::player::Player;

fn main() {
    use env_logger::init;
    println!("Hello, world!");

    let mut player_name = String::new();
    println!("bisey söyle pasam");
    let result = stdin().read_line(&mut player_name);
    match result {
        Ok(l) => {
            info!("{} byte veri alındı", l);
        }
        Err(e) => {
            error!("{}", e);
            exit(-1)
        }
    }
    // let oyuncu = Player{
    //     name:   player_name.trim(),
    //     level: Level::Beginner,
    // };

    let mut oyuncu = Player::new(player_name.trim(), Level::Pro(1));
    println!("{}", oyuncu);
    apply_promotion( &mut oyuncu);
    println!("{}", oyuncu);


}

fn apply_promotion(p: &mut Player){
    p.level = Level::Senior;
    println!("{}", p);
}
