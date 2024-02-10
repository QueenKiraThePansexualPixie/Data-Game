
struct Date {
    year: i64,
    month: u8,
    day: u8,
}

struct Character {
    name: String,
    dob: Date,
    /// TODO: TYPES !!!
    traits: Vec<CharacterTrait>,
    ability: Vec<CharacterAbilities>,
    skill: Vec<CharacterSkill>,
    languages: Vec<Language>,
    inventory: Inventory,
}
