use std::collections::HashSet;
use uuid::Uuid;

/// Represents a single recipe one would find in a cookbook.
pub struct Recipe {
    /// The id of the recipe
    id: Uuid,
    /// The name of the recipe
    name: String,
    /// The difficulty rating of the recipe
    difficulty: Difficulty,
    /// The estimated duration of the recipe in minutes
    duration: u16,
    /// The description of the recipe
    description: String,
    /// The ingredients needed for the recipe
    ingredients: HashSet<Ingredient>,
    /// The directions to create the recipe
    directions: String,
    /// Optional tags that help describe the recipe
    tags: HashSet<RecipeTag>,
    /// The picture of the recipe
    img: Vec<u8>,
}

impl Recipe {
    pub fn new(
        id: Uuid,
        name: String,
        difficulty: Difficulty,
        duration: u16,
        description: String,
        ingredients: HashSet<Ingredient>,
        directions: String,
        tags: HashSet<RecipeTag>,
        img: Vec<u8>
    ) -> Self {
        Self { id, name, difficulty, duration, description, ingredients, directions, tags, img }
    }
    pub fn builder() -> RecipeBuilder {
        RecipeBuilder::new()
    }
}

pub struct RecipeBuilder {
    /// The id of the recipe, yet to be set
    id: Option<Uuid>,
    /// The name of the recipe, yet to be set
    name: Option<String>,
    /// The difficulty rating of the recipe, yet to be set
    difficulty: Option<Difficulty>,
    /// The estimated duration of the recipe in minutes, yet to be set
    duration: Option<u16>,
    /// The description of the recipe, yet to be set
    description: Option<String>,
    /// The ingredients needed for the recipe, yet to be set
    ingredients: HashSet<Ingredient>,
    /// The directions to create the recipe, yet to be set
    directions: Option<String>,
    /// Optional tags that help describe the recipe, yet to be set
    tags: HashSet<RecipeTag>,
    /// The picture of the recipe, yet to be set
    img: Option<Vec<u8>>,
}

impl RecipeBuilder {
    fn new() -> Self {
        Self {
            id: None,
            name: None,
            difficulty: None,
            duration: None,
            description: None,
            ingredients: HashSet::new(),
            directions: None,
            tags: HashSet::new(),
            img: None
        }
    }
    fn id(mut self, id: Uuid) -> Self {
        self.id = Some(id);
        self
    }

    fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    fn difficulty(mut self, difficulty: Difficulty) -> Self {
        self.difficulty = Some(difficulty);
        self
    }

    fn duration(mut self, duration: u16) -> Self {
        self.duration = Some(duration);
        self
    }

    fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    fn directions(mut self, directions: String) -> Self {
        self.directions = Some(directions);
        self
    }

    fn ingredient(mut self, ingredient: Ingredient) -> Self {
        self.ingredients.insert(ingredient);
        self
    }

    fn tag(mut self, tag: RecipeTag) -> Self {
        self.tags.insert(tag);
        self
    }

    fn img(mut self, img: Vec<u8>) -> Self {
        self.img = Some(img);
        self
    }

    fn build(mut self) -> Recipe {
        Recipe {
            id: self.id.take().expect("cannot build `Recipe` struct without id set"),
            name: self.name.take().expect("cannot build `Recipe` struct without name set"),
            difficulty: self.difficulty.take().expect("cannot build `Recipe` without difficulty set"),
            duration: self.duration.take().expect("cannot build `Recipe` without duration set"),
            description: self.description.take().expect("cannot build `Recipe` without description set"),
            directions: self.directions.take().expect("cannot build `Recipe` without directions set"),
            ingredients: self.ingredients,
            tags: self.tags,
            img: self.img.take().or(Some(Vec::new())).unwrap()
        }
    }
}

/// Represents the difficulty of a recipe on a scale of 1 to 4.
/// The `Easy` variant being the easiest kind of recipe to make and `Expert` variant being
/// the most difficult kind of recipe to make.
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
    Expert,
}

/// An ingredient for for a recipe.
pub struct Ingredient {
    id: Uuid,
    name: String,
    unit: String,
    measurement: String,
}

/// A wrapper type for a `String`, that represents any optional tags for a recipe.
pub struct RecipeTag {
    tag: String
}

