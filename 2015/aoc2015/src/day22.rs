mod rpggamemagic;

use crate::rpggamemagic::{
    boss_attack, drain, magic_missile, poison, recharge, shield, transitions, ucs, GameState,
};

// now we will apply ucs to compute best strategy to win the game

pub fn main() {
    println!("This game will make me quit");

    let state = GameState::new(500, 50, 58);

    let answer1 = ucs(&state);
    println!("answer1 = {answer1}");
}
