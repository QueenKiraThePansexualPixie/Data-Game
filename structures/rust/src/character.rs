
/// Represents a character.
pub struct Character {
    name: String,
    aliases: Vec<String>,
    dob: Date,
    species: Species,
    occupation: Occupation,
    skill: Vec<Skill>,
    ability: Vec<Ability>,
    traits: Vec<Trait>,
    languages: Vec<Language>,
    relationships: Vec<(Character, String)>,
    affiliations: Vec<(Organisation, String)>,
    locations: Vec<(Location, String)>,
    inventory: Inventory,
    notes: Vec<Note>,
}

// Represents a possible condition that a [`Character`] could have.
pub struct Condition {
    name: String,
    description: String,
}

/// Represents a [`Character`] skill - something anyone can try and build up, 
/// like swimming or cooking.
pub struct Skill {
    name: String,
    modifier: Proficiency,
    description: String,
}

/// Represents a [`Character`] ability - something most people won't be able to 
/// have, let alone build upon, such as a superpower or a D&D cantrip.
pub struct Ability {
    name: String,
    levels: u8,
    actions: Vec<Action>,
    descriptions: Vec<String>,
}

/// Represents a [`Character`] trait - something you can be born with or 
/// something you develop, but always something that affects certain aspects of 
/// a character, usually in a balancing act, such as an increased perception 
/// skill, traded off for a decreased passive perception.
pub struct Trait {
    name: String,
    levels: u8,
    actions: Vec<Action>,
    descriptions: Vec<String>,
}

/// Represents a learnable language that a character can speak, read, write, 
/// and/or translate using a device, ability or spell.
pub struct Language {
    name: String,
    dictionary: HashMap<String, String>,
    difficulty: u8,
}

/// Represents a player's inventory.
pub struct Inventory {
    items: Vec<Item>,
    containers: Vec<ItemContainer>,
}

/// Represents an item.
pub struct Item {
    name: String,
    actions: Option<Vec<Action>>,
    tags: Vec<ItemTag>,
    rarity: Rarity,
    description: String,
    weight: u32,
    cost: u32,
}

/// Rperesents a portable storage container, like a backpack.
pub struct ItemContainer {
    name: String,
    rarity: Rarity,
    items: Vec<Item>,
    weight: u32,
    cost: u32,
}
