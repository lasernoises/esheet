use serde::{Deserialize, Serialize};

// Maybe make a newtype from this.
// With methods for the string and stuff.
pub type Level = u8;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CharSheet {
    pub name: String,

    pub level: Level,
    pub ethnicity: String,
    pub species: Species,
    pub size: u8, // in centimeters

    pub class: Class,

    pub plan: [LevelPlan; 20],

    pub inventory: Inventory,

    #[serde(skip)]
    pub computed: Computed,

    #[serde(skip)]
    pub add_item: AddItem,
}

impl Default for CharSheet {
    fn default() -> Self {
        Self {
            name: Default::default(),
            level: 1,
            ethnicity: Default::default(),
            species: Default::default(),
            size: 170,

            class: Default::default(),

            plan: Default::default(),

            inventory: Default::default(),

            computed: Default::default(),

            add_item: Default::default(),
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct AddItem {
    pub item_search: String,
    pub custom_item_name: String,
    pub custom_item_bulk: u32,
}

impl CharSheet {
    // This should be cheap enough that it can be done in every frame.
    pub fn compute(&mut self) {
        self.computed = Default::default();

        for l in 1..=self.level {
            self.computed
                .skills
                .apply_training(&self.plan[(l - 1) as usize].skill_training);

            self.computed
                .abilities
                .apply_boosts(&self.plan[(l - 1) as usize].ability_boosts)
        }

        self.computed
            .skills
            .compute_modifiers(self.level, &self.computed.abilities);
    }

    pub fn level_plan(&mut self, level: Level) -> &mut LevelPlan {
        &mut self.plan[(level - 1) as usize]
    }
}

#[derive(Clone, Debug, Default)]
pub struct Computed {
    pub fortitude: Training,
    pub reflex: Training,
    pub will: Training,

    pub abilities: Abilities,
    pub skills: crate::app::Skills,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Abilities {
    pub strength: i8,
    pub dexterity: i8,
    pub mind: i8,
    pub charisma: i8,
}

impl Abilities {
    pub fn apply_boosts(&mut self, boosts: &AbilityBoosts) {
        self.strength += boosts.strength as i8;
        self.dexterity += boosts.dexterity as i8;
        self.mind += boosts.mind as i8;
        self.charisma += boosts.charisma as i8;
    }
}

#[derive(Copy, Clone, Debug, Default, Deserialize, Serialize)]
#[non_exhaustive]
pub enum Species {
    #[default]
    Human,
}

#[derive(Copy, Clone, Debug, Default, Deserialize, Serialize)]
pub enum Gender {
    #[default]
    Female,
    Male,
}

#[derive(Copy, Clone, Debug, Default, Deserialize, Serialize)]
pub enum Training {
    #[default]
    None,
    Trained,
    Expert,
    Master,
    Legendary,
}

impl Training {
    pub fn label(&self) -> &'static str {
        use Training::*;

        match self {
            None => "U",
            Trained => "T",
            Expert => "E",
            Master => "M",
            Legendary => "L",
        }
    }

    pub fn increase(&mut self) {
        use Training::*;

        *self = match self {
            None => Trained,
            Trained => Expert,
            Expert => Master,
            Master => Legendary,
            Legendary => Legendary,
        }
    }

    /// Pathfinder Core Rulebook Page 13:
    /// Proficiency is a system that measures a character’s aptitude
    /// at a specific task or quality, and it has five ranks: untrained,
    /// trained, expert, master, and legendary. Proficiency gives
    /// you a bonus that’s added when determining the following
    /// modifiers and statistics: AC, attack rolls, Perception, saving
    /// throws, skills, and the effectiveness of spells. If you’re
    /// untrained, your proficiency bonus is +0. If you’re trained,
    /// expert, master, or legendary, your proficiency bonus equals
    /// your level plus 2, 4, 6, or 8, respectively.
    pub fn proficiency_bonus(&self, level: Level) -> u8 {
        use Training::*;

        match self {
            None => 0,
            Trained => level + 2,
            Expert => level + 4,
            Master => level + 6,
            Legendary => level + 8,
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Inventory {
    pub platinum: u32,
    pub gold: u32,
    pub silver: u32,
    pub copper: u32,

    pub items: Vec<(Item, u32)>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Item {
    pub id: u64,
    pub name: String,
    pub bulk: u32,
}

/// This contains all of the things a levelup can contain
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LevelPlan {
    pub skill_training: crate::app::SkillTraining,
    pub ability_boosts: AbilityBoosts,

    pub feats: u8,
}

// Could also be a bit field.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AbilityBoosts {
    pub strength: bool,
    pub dexterity: bool,
    pub mind: bool,
    pub charisma: bool,
}

struct Feat {
    id: u64,
    name: &'static str,

    description: &'static str,

    class: Class,
}

#[derive(Copy, Clone, Debug, Default, Deserialize, Serialize)]
pub enum Class {
    #[default]
    Engineer,
    Scout,
}

const FEATS: &[Feat] = &[Feat {
    id: 1,
    name: "Quick Engineering",
    description: "Bla bla bla",
    class: Class::Engineer,
}];
