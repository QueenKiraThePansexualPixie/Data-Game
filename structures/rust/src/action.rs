use std::cmp;
use std::ops::Add;
use rand::Rng;

/// Used for creasting dice to roll (E.g. `1d6`, `10d10`, ...)
pub struct Dice {
    /// The count of how many dice with the specified number of 
    /// [sides](Dice::sides) will be [rolled](Dice::roll).
    count: u32,
    sides: u8,
}

impl Dice {
    /// Create a new [`Dice`] object
    pub fn new(count: u32, sides: u8) -> Dice {
        Dice { count, sides }
    }

    /// Roll the dice with the current data, plus a vantage and the modifier
    pub fn roll(&self, vantage: Vantage, modifier: Modifier) -> DiceResult {
        match vantage {
            Vantage::Nonvantage => self.base_roll() + modifier,
            Vantage::Advantage => cmp::max(self.base_roll() + modifier, self.base_roll() + modifier),
            Vantage::Disadvantage => cmp::min(self.base_roll() + modifier, self.base_roll() + modifier),
        }
    }

    /// Roll the dice with the current data, but no vantage
    fn base_roll(&self) -> u32 {
        rand.thread_rng().gen_range(1..=self.sides) * self.count
    }

    /// Roll the dice with the current data, with no vantage but including a 
    /// modifier.
    fn modifier_roll(&self, modifier: Modifier) -> u32 {
        self.base_roll() + modifier
    }
}

pub struct Modifier {
    value: u32,
    modifier_type: ModifierType,
}

impl Modifier {
    pub fn new(value: u32, modifier_type: ModifierType) -> Modifier {
        Modifier { value, modifier_type }
    }

    pub fn value(&self) -> u32 {
        self.value.clone()
    }

    pub fn modifier_type(&self) -> ModifierType {
        self.modifier_type.clone()
    }
}

impl Add<Rhs = u32> for Modifier {
    type Output = DiceResult;

    fn add(&self, rhs: u32) -> DiceResult {
        DiceResult::new(
            value: self.value + rhs,
            result_type: modifier.modifier_type,
        )
    }
}

impl Add<Rhs = Modifier> for u32 {
    type Output = DiceResult;

    fn add(&self, rhs: Modifier) -> DiceResult {
        DiceResult::new(
            value: self + rhs.value(),
            result_type: rhs.modifier_type(),
        )
    }
}

/// Used to represent whether a dice roll has an 
/// [advantage](Vantage::Advantage), a 
/// [disadvantage](Vantage::Disadvantage), or 
/// [neither](Vantage::Nonvantage).
///
/// [`Nonvantage`](Vantage::Nonvantage): Default, no 2nd roll
/// [`Advantage`](Vantage::Advantage): 2nd roll, take the higher result
/// [`Disadvantage`](Vantage::Disadvantage): 2nd roll, take the lower result
pub enum Vantage {
    /// `Nonvantage`: Default, no 2nd roll
    Nonvantage,
    /// `Advantage`: 2nd roll, take the higher result
    Advantage,
    /// `Disadvantage`: 2nd roll, take the lower result
    Disadvantage,
}

/// U
pub struct Action {
    dice: Vec<Dice>
    vantage: Vantage
    descriptions: Vec<(u8, String)>
}
