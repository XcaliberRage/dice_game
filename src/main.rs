/*
Peter has nine four-sided (pyramidal) dice, each with faces numbered 1, 2, 3, 4.
Colin has six six-sided (cubic) dice, each with faces numbered 1, 2, 3, 4, 5, 6.

Peter and Colin roll their dice and compare totals: the highest total wins. The result is a draw if the totals are equal.

What is the probability that Pyramidal Pete beats Cubic Colin? Give your answer rounded to seven decimal places in the form 0.abcdefg


How many ways can Peter's dice produce a result that can beat Colin's dice?

By knowing how many ways each number can be made, we can start to determine the chances of getting a given number

For any given number Colin can get, we want to know the probability Peter has of getting a higher result.

 */

use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};

#[derive(Clone)]
struct Dice {
    sides: u32,
    faces: Vec<u32>,
    average: f32,
}

impl Dice {
    pub fn new(sides: u32) -> Self {
        let mut faces = Vec::new();

        for i in 0..sides {
            faces.push(i + 1);
        }

        Self {
            sides,
            faces,
            average: (sides + 1) as f32 / 2.0,
        }
    }
}

impl Display for Dice {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "D{}: {:?}", self.sides, self.faces)
    }
}

struct Player {
    dice_type: Dice,
    dice_count: u32,
}

// A player in this instance is a conceptual set of dice
impl Player {
    pub fn new(dice_type: Dice, dice_count: u32) -> Self {
        Self {
            dice_type,
            dice_count,
        }
    }

    // Returns an ordered, keyed array of possible sums and the number of ways each sum can be generated by this set of dice
    pub fn get_sums(&self) -> BTreeMap<u32, u32> {
        let mut sums = BTreeMap::new();

        // Can't roll higher than all the dice showing their highest value.
        let max = self.dice_type.sides * self.dice_count;

        // by making a lookup reference, we can store previously marked combinations, saving some time and memory
        for val in 1..max + 1 {
            let mut lookup = Vec::new();

            for _ in 0..(self.dice_count + 1) {
                let mut throw = Vec::new();
                for _ in 0..(val + 1) {
                    throw.push(0);
                }
                lookup.push(throw);
            }
            let t = self.get_combinations(self.dice_count as i32, val as i32, &mut lookup);
            sums.insert(val, t);
        }
        sums
    }

    // Recursively seeks the number of ways to get the given value using the set of given dice.
    // Lookup is referenced so that we don't repeat ourselves unnecessarily.
    //
    // * `n` - number of dice
    // * `f` - number of faces (always 1-f in value)
    // * `x` - target sum
    pub fn get_combinations(&self, n: i32, x: i32, lookup: &mut Vec<Vec<u32>>) -> u32 {
        let f = self.dice_type.sides as i32;

        if n == x {
            return 1;
        } else if n == 0 || x < n || f * n < x {
            return 0;
        }

        if lookup[n as usize][x as usize] == 0 {
            for i in 1..(f + 1) {
                lookup[n as usize][x as usize] += self.get_combinations(n - 1, x - i, lookup);
            }
        }

        lookup[n as usize][x as usize]
    }
}

fn main() {
    let d_four = Dice::new(4);
    let d_six = Dice::new(6);

    let peter = Player::new(d_four, 9);
    let colin = Player::new(d_six, 6);

    let probability = a_beats_b(peter, colin);

    println!("The chance for Peter to beat Colin is {:.7}", probability);
}

// Returns the probability in float that player A will beat player B given one roll of their dice.
fn a_beats_b(a: Player, b: Player) -> f64 {
    let a_sums = a.get_sums();
    let b_sums = b.get_sums();

    // Probability in f64 that A beats B's roll of a given number.
    let mut a_chance = BTreeMap::new();
    let a_poss = a.dice_type.sides.pow(a.dice_count) as f64;

    let mut b_chance = BTreeMap::new();
    let b_poss = b.dice_type.sides.pow(b.dice_count) as f64;

    // For each roll B can get, save the chance he can roll that number and also the chance A can beat it.
    for b_roll in b_sums {
        // Just ignore impossible rolls
        if b_roll.1 == 0 {
            continue;
        }

        // Same
        if b_roll.0 < b.dice_count {
            continue;
        }

        b_chance.insert(b_roll.0, b_roll.1 as f64 / b_poss);

        // If it's less than A's minimum, A is guaranteed to win
        if b_roll.0 <= a.dice_count {
            a_chance.insert(b_roll.0, 1.0);
            continue;
        }

        // If it's greater or equal than what A can possibly roll, then A cannot possibly win
        // In this case, literally only if B rolls max roll
        if b_roll.0 >= a.dice_count * a.dice_type.sides {
            a_chance.insert(b_roll.0, 0.0);
            continue;
        }

        let mut sum = 0;
        // Now just slap together all the different combinations A can roll to beat B's roll
        for a_roll in a_sums.iter() {
            if *a_roll.0 > b_roll.0 {
                sum += a_roll.1;
            }
        }

        // And divide them by the total number of combinations A can roll (num_faces^num_dice)
        let prob = sum as f64 / a_poss;
        a_chance.insert(b_roll.0, prob);
    }

    let mut probability = 0.0;
    // Now just slap together all the probabilities of any given result B can get using A's chance to beat that specific roll as a multiplier.
    for roll in b_chance {
        let b_by_a = roll.1 * a_chance[&roll.0];
        probability = if probability == 0.0 {
            b_by_a
        } else {
            probability + b_by_a
        }
    }

    probability
}
