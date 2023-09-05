// struct representing the weapon
#[derive(Debug)]
pub enum EquipementType {
    WEAPON,
    ARMOR,
    RING,
}

#[derive(Debug)]
pub struct Equipement {
    etype: EquipementType,
    cost: i32,
    damage: i32,
    armor: i32,
}

impl Equipement {
    #[inline]
    pub fn new(etype: EquipementType, cost: i32, damage: i32, armor: i32) -> Self {
        Self {
            etype,
            cost,
            damage,
            armor,
        }
    }
}

/* Structure for the shop
 */

#[derive(Debug)]
pub struct Shop {
    pub weapons: Vec<Equipement>,
    pub armors: Vec<Equipement>,
    pub rings: Vec<Equipement>,
}

// Vector for the values of the shop

impl Shop {
    #[inline]
    pub fn new() -> Self {
        let costs_all: Vec<i32> = vec![
            8, 10, 25, 40, 74, 13, 31, 53, 75, 102, 25, 50, 100, 20, 40, 80,
        ];

        let damages_all: Vec<i32> = vec![4, 5, 6, 7, 8, 0, 0, 0, 0, 0, 1, 2, 3, 0, 0, 0];

        let armors_all: Vec<i32> = vec![0, 0, 0, 0, 0, 1, 2, 3, 4, 5, 0, 0, 0, 1, 2, 3];

        let types_all: Vec<i32> = vec![0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2];

        let mut weapons = Vec::new();
        let mut armors = Vec::new();
        let mut rings = Vec::new();

        for i in 0..types_all.len() {
            let t = types_all[i];
            let cost = costs_all[i];
            let damage = damages_all[i];
            let armor = armors_all[i];

            match t {
                0 => weapons.push(Equipement::new(EquipementType::WEAPON, cost, damage, armor)),
                1 => armors.push(Equipement::new(EquipementType::ARMOR, cost, damage, armor)),
                _ => rings.push(Equipement::new(EquipementType::RING, cost, damage, armor)),
            }
        }
        // Create the shop with the given instructions
        Self {
            weapons,
            armors,
            rings,
        }
    }
}
#[derive(Debug, Clone)]
pub struct Player {
    hits: i32,
    damage: i32,
    armor: i32,
    pub cost: i32, // cost to create the player
}

impl Player {
    #[inline]
    pub fn new(hits: i32, damage: i32, armor: i32, cost: i32) -> Self {
        Self {
            hits,
            damage,
            armor,
            cost,
        }
    }
    #[inline]
    pub fn from(shop: &Shop, chosen: Vec<i32>) -> Self {
        // create a player by shoosing iterms from the shop
        // the indicator -1 indicate absense
        let mut damage = 0; // my damage
        let mut armor = 0; // my armor
        let hits = 100; // my hits
        let mut cost = 0; // cost to create the player

        // check weapon
        let i = chosen[0];
        if i != -1 {
            let i = i as usize;
            damage += shop.weapons[i].damage;
            cost += shop.weapons[i].cost;
        }

        let j = chosen[1];

        if j != -1 {
            let j = j as usize;
            armor += shop.armors[j].armor;
            cost += shop.armors[j].cost;
        }

        let i = chosen[2];

        if i != -1 {
            let i = i as usize;
            armor += shop.rings[i].armor;
            damage += shop.rings[i].damage;
            cost += shop.rings[i].cost;
        }

        // check if we have a second ring
        if chosen.len() == 4 {
            let i = chosen[3];

            if i != -1 {
                let i = i as usize;
                armor += shop.rings[i].armor;
                damage += shop.rings[i].damage;
                cost += shop.rings[i].cost;
            }
        }

        Self {
            hits,
            damage,
            armor,
            cost,
        }
    }

    pub fn play_against(&mut self, other: &Self) -> bool {
        // check if I would win against another player (generally the boss)
        let mut other = other.clone();
        while self.hits > 0 && other.hits > 0 {
            // me attackhing the player
            other.hits -= (self.damage - other.armor).max(1);

            if other.hits <= 0 {
                return true;
            }

            // now the other player attacks me

            self.hits -= (other.damage - self.armor).max(1);

            if self.hits <= 0 {
                return false;
            }
        }

        false
    }
}
