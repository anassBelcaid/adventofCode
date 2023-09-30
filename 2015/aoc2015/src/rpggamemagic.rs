// module for storing the state of the game.
use std::collections::{BinaryHeap, HashSet};
static BOSS_DAMAGE: i32 = 9;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct GameState {
    pub mana: i32,
    pub hits: i32,
    pub boss: i32,
    pub shield: i32,
    pub player_turn: bool,
    pub poison: i32,
    pub recharge: i32,
    pub used_mana: i32,
}

// implementing the order on the used mana

impl PartialOrd for GameState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Some(other.used_mana.cmp(&self.used_mana));
    }
}

impl Ord for GameState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.used_mana.cmp(&other.used_mana);
    }
}

impl GameState {
    #[inline]
    pub fn new(mana: i32, hits: i32, boss: i32) -> Self {
        Self {
            mana,
            hits,
            boss,
            shield: 0,
            poison: 0,
            recharge: 0,
            used_mana: 0,
            player_turn: true,
        }
    }
}

pub fn poison_effect(state: &mut GameState) {
    // apply the state of poison

    if state.poison > 0 {
        state.boss -= 3;
        state.boss = state.boss.max(0);
        state.poison -= 1;
    }
}

pub fn shield_effect(state: &mut GameState) {
    // function to apply the shield effect

    // simply reduce the counter of the shield
    if state.shield > 0 {
        state.shield -= 1;
    }
}

pub fn recharge_effect(state: &mut GameState) {
    // function to imply the recharge the state
    if state.recharge > 0 {
        state.recharge -= 1;
        state.mana += 101;
    }
}

pub fn cast_poison(state: &GameState) -> Option<GameState> {
    // compute the transition after casting poison

    let mut s = state.clone();

    poison_effect(&mut s);
    recharge_effect(&mut s);
    shield_effect(&mut s);

    if s.mana >= 173 && s.poison == 0 {
        s.mana -= 173;
        s.used_mana += 173;
        s.poison = 6;
        s.player_turn = false;
        return Some(s);
    }
    None
}

pub fn boss_attack(state: &GameState) -> Option<GameState> {
    let mut s = state.clone();

    // apply the effects
    poison_effect(&mut s);
    recharge_effect(&mut s);
    shield_effect(&mut s);

    // now the boss attach ( he has to survive the effects)
    if s.boss > 0 {
        // compute the the player armor which depends on the shield
        let armor = if s.shield > 0 { 7 } else { 0 };
        s.hits -= (BOSS_DAMAGE - armor).max(1);
        s.hits = s.hits.max(0);
        s.player_turn = true;
    }
    return Some(s);
}

pub fn magic_missile(state: &GameState) -> Option<GameState> {
    // transition state if the player casts magic missile
    let mut s = state.clone();

    // apply the effect
    poison_effect(&mut s);
    recharge_effect(&mut s);
    shield_effect(&mut s);

    // An important detail is that playe cast if the player doesn't die
    if s.boss == 0 {
        return Some(s);
    }
    // now we need to cast
    if s.mana >= 53 {
        s.mana -= 53;
        s.used_mana += 53;
        s.boss -= 4;
        s.boss = s.boss.max(0);
        s.player_turn = false;
        return Some(s);
    }

    None
}

pub fn drain(state: &GameState) -> Option<GameState> {
    // function to apply the drain effect
    let mut s = state.clone();

    // apply the effect
    poison_effect(&mut s);
    recharge_effect(&mut s);
    shield_effect(&mut s);

    if s.boss == 0 {
        return Some(s);
    }

    if s.mana >= 73 {
        s.mana -= 73;
        s.used_mana += 73;
        s.boss -= 2;
        s.boss = s.boss.max(0);
        s.hits += 2;
        s.player_turn = false;
        return Some(s);
    }
    None
}

pub fn shield(state: &GameState) -> Option<GameState> {
    // function to cast shield

    let mut s = state.clone();

    // Apply the effects
    poison_effect(&mut s);
    recharge_effect(&mut s);
    shield_effect(&mut s);

    if s.boss == 0 {
        return Some(s);
    }

    if s.mana >= 113 && s.shield == 0 {
        s.mana -= 113;
        s.used_mana += 113;
        s.shield = 6;
        s.player_turn = false;
        return Some(s);
    }
    None
}

pub fn poison(state: &GameState) -> Option<GameState> {
    // function to apply the poison effect
    let mut s = state.clone();

    poison_effect(&mut s);
    recharge_effect(&mut s);
    shield_effect(&mut s);

    if s.boss == 0 {
        return Some(s);
    }

    if s.mana >= 173 && s.poison == 0 {
        s.mana -= 173;
        s.used_mana += 173;
        s.poison = 6;
        s.player_turn = false;
        return Some(s);
    }
    None
}

pub fn recharge(state: &GameState) -> Option<GameState> {
    // function to check if we could apply the

    let mut s = state.clone();

    // applying the effects
    poison_effect(&mut s);
    recharge_effect(&mut s);
    shield_effect(&mut s);

    if s.boss == 0 {
        return Some(s);
    }

    if s.mana >= 229 && s.recharge == 0 {
        s.mana -= 229;
        s.used_mana += 229;
        s.player_turn = false;
        s.recharge = 5;
        return Some(s);
    }

    None
}

pub fn transitions(state: &GameState) -> Vec<GameState> {
    // function to return a set of transitions states from the current one
    let mut results = Vec::new();

    if state.player_turn {
        // check if we could cast magic_missile
        if let Some(v) = magic_missile(state) {
            results.push(v);
        }

        // check if we could cast cdrain
        if let Some(v) = drain(state) {
            results.push(v);
        }

        // check if we could cast shield
        if let Some(v) = shield(state) {
            results.push(v);
        }

        // check if we could cast poison
        if let Some(v) = poison(state) {
            results.push(v);
        }

        // finally cast recharge
        if let Some(v) = recharge(state) {
            results.push(v);
        }
    } else {
        results.push(boss_attack(&state).unwrap());
    }
    results
}

//
//
pub fn ucs(state: &GameState) -> i32 {
    // function to compute the minimum cost to beat the boss
    let mut queue = BinaryHeap::new();

    let mut visited = HashSet::new();

    // pushing the first node
    queue.push(state.clone());
    visited.insert(state.clone());

    while !queue.is_empty() {
        // getting the most closest node
        let state = queue.pop().unwrap();
        // println!("got state {:?} ", state);

        if state.boss == 0 {
            return state.used_mana;
        }

        // if player dies no continutation
        if state.hits == 0 {
            continue;
        }

        // check the other transition
        for trans in transitions(&state) {
            if !visited.contains(&trans) {
                queue.push(trans.clone());
                visited.insert(trans);
            }
        }
    }
    0
}
