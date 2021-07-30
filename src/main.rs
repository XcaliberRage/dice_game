use fraction::*;
/*
Peter has nine four-sided (pyramidal) dice, each with faces numbered 1, 2, 3, 4.
Colin has six six-sided (cubic) dice, each with faces numbered 1, 2, 3, 4, 5, 6.

Peter and Colin roll their dice and compare totals: the highest total wins. The result is a draw if the totals are equal.

What is the probability that Pyramidal Pete beats Cubic Colin? Give your answer rounded to seven decimal places in the form 0.abcdefg


How many ways can Peter's dice produce a result that can beat Colin's dice?

By knowing how many ways each number can be made, we can start to determine the chances of getting a given number

For any given number Colin can get, we want to know the probability Peter has of getting a higher result.

 */

use std::fmt::{Display, Formatter};
use std::collections::{HashMap, BTreeMap};

#[derive(Clone)]
struct Dice {
    sides: u32,
    faces: Vec<u32>,
    average: f32
}

impl Dice {
    pub fn new(sides: u32) -> Self {

        let mut faces = Vec::new();

        for i in 0..sides {
            faces.push(i+1);
        }

        Self {
            sides,
            faces,
            average: (sides + 1) as f32 / 2.0
        }
    }
}

impl Display for Dice {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "D{}: {:?}", self.sides, self.faces)
    }
}

struct Player {
    name: String,
    dice_type: Dice,
    dice_count: u32,
}

// A player in this instance is a conceptual set of dice
impl Player {
    pub fn new(name: String, dice_type: Dice, dice_count: u32) -> Self {
        Self {
            name,
            dice_type,
            dice_count
        }
    }

    pub fn get_sums(&self) -> BTreeMap<u32, u32> {
        let mut sums = BTreeMap::new();

        let max = self.dice_type.sides * self.dice_count;


        for val in 1..max + 1 {
            let mut lookup = Vec::new();

            for _ in 0..(self.dice_count + 1) {
                let mut throw = Vec::new();
                for _ in 0..(val+1) {
                    throw.push(0);
                }
                lookup.push(throw);

            }
            let t = self.get_combinations(self.dice_count as i32, val as i32, &mut lookup);
            sums.insert(val, t);

        }
        sums
    }

    // Recursively seeks the number of ways to get the given value using the set of given dice
    pub fn get_combinations(&self, n: i32, x: i32, lookup: &mut Vec<Vec<u32>>) -> u32 {

        // n = number of dice
        // f = number of faces (always 1-f in value)
        // x = target sum
        let f = self.dice_type.sides as i32;

        if n == x {
            return 1
        } else if n == 0 || x < n || f * n < x {
            return 0
        }

        if lookup[n as usize][x as usize] == 0 {
            for i in 1..(f+1) {
                lookup[n as usize][x as usize] += self.get_combinations(n-1, x-i, lookup);
            }
        }

        lookup[n as usize][x as usize]
    }
}

#[derive(Clone)]
struct Combination {

    dice_type: Dice,
    dice_count: u32,
    dice: Vec<u32>,
    sum: u32

}

impl Combination {
    pub fn new(dice_type: Dice, dice_count: u32) -> Self {

        let mut combo = Vec::new();

        for i in 0..dice_count {
            combo.push(1);
        }

        let mut sum = 0;

        for dice in combo.iter().clone() {
            sum += dice;
        }

        Self {
            dice_type,
            dice_count,
            dice: combo,
            sum
        }
    }

    pub fn incr(&mut self, mut pos: usize) {

        if pos >= self.dice.len() {
            panic!("Tried to create more combinations than possible for {}", self.dice_type);
        }

        if self.dice[pos] != self.dice_type.sides {
            self.dice[pos] += 1;
        } else {
            self.dice[pos] = 1;
            pos += 1;
            self.incr(pos);
        }

        self.sum();

    }

    pub fn sum(&mut self) {

        self.sum = 0;

        for dice in self.dice.iter() {
            self.sum += dice;
        }

    }
}

impl Display for Combination {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}d{}: {:?} = {}",
               self.dice_count,
               self.dice_type.sides,
               self.dice,
               self.sum)
    }
}

impl PartialEq for Combination {
    fn eq(&self, other: &Self) -> bool {

        if self.dice_type.faces != other.dice_type.faces {
            return false;
        }

        if self.dice_count != other.dice_count {
            return false;
        }

        if self.sum != other.sum {
            return false;
        }

        let d_len = self.dice.len();
        let o_len = other.dice.len();

        if d_len != o_len {
            return false;
        }

        for i in 0..d_len {
            if self.dice[i] != other.dice[i] {
                return false;
            }
        }

        true
    }
}


fn main() {

    let d_four = Dice::new(4);
    let d_six = Dice::new(6);

    println!("{}", d_four);
    println!("{}", d_six);

    let peter = Player::new("Peter".parse().unwrap(), d_four, 9);
    let colin = Player::new("Colin".parse().unwrap(), d_six, 6);

    let peter_sums = peter.get_sums();
    let colin_sums = colin.get_sums();

    // Probability in f64 that peter beats colin's roll of a given number.
    let mut peter_chance = BTreeMap::new();
    let peter_poss = peter.dice_type.sides.pow(peter.dice_count) as f64;

    for colin_roll in colin_sums {
        if colin_roll.1 == 0 {
            continue;
        }

        if colin_roll.0 <= peter.dice_count {
            peter_chance.insert(colin_roll.0, 1.0);
            continue;
        }

        if colin_roll.0 >= peter.dice_count * peter.dice_type.sides {
            peter_chance.insert(colin_roll.0, 0.0);
            continue;
        }

        let mut sum = 0;
        for peter_roll in peter_sums.iter() {
            if *peter_roll.0 > colin_roll.0 {
                sum += peter_roll.1;
            }
        }

        let prob = sum as f64 / peter_poss;
        peter_chance.insert(colin_roll.0, prob);
    }

    println!("Peter's chances to beat each of Colin's rolls:");
    for chance in peter_chance.iter() {
        println!("{}: -> {:.7}", chance.0, chance.1);
    }

}

fn make_combos(player: Player) -> Vec<Combination> {

    let d_type = player.dice_type;
    let d_ct = player.dice_count;

    // All possible combinations of dice
    let total_combos = d_type.sides.pow(player.dice_count) as usize;

    let mut all_combos = vec![Combination::new(d_type.clone(), d_ct)];

    while all_combos.len() != total_combos {

        if all_combos.len() > total_combos {
            panic!("Added more combos than possible!");
        }

        let mut new_combo = all_combos.last().unwrap().clone();

        while all_combos.contains(&new_combo) {

            new_combo.incr( 0_usize);

        }

        all_combos.push(new_combo.clone());
        println!("Push {}", new_combo);

    }

    all_combos
}