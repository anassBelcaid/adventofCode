use itertools::Itertools;
use rpggame::Player;
pub mod rpggame;
use crate::rpggame::Shop;

pub fn part1() -> i32 {
    // First we need to create all the possible armors that I should use
    let shop = Shop::new();
    let n = shop.weapons.len() as i32;
    let m = shop.armors.len() as i32;
    let k = shop.rings.len() as i32;

    // nwe we will consider a single player
    let boss = Player::new(103, 9, 2, 0);

    // loop on the weapon
    let mut all_configuration = Vec::new();
    for i in 0..=n {
        for j in 0..=m {
            // combination with zero ring
            all_configuration.push(vec![i - 1, j - 1, -1]);

            //combination with a single ring
            for k in 1..=k {
                all_configuration.push(vec![i - 1, j - 1, k - 1]);
            }

            // now combination with two rings
            for comb in (1..=k).combinations(2) {
                all_configuration.push(vec![i - 1, j - 1, comb[0] - 1, comb[1] - 1]);
            }
        }
    }

    let mut min_cost = i32::MAX;
    // checking all the configuraation
    for config in all_configuration {
        // Create the player
        let mut player = Player::from(&shop, config);

        if player.play_against(&boss) {
            min_cost = min_cost.min(player.cost);
        }
    }

    min_cost
}
pub fn main() {
    let answer1 = part1();
    println!("Part I = {answer1}");
}
