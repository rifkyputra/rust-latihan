pub use crate::world;

pub struct Pokemon {
    id: String,
    pub name: String,
    pub level: i8,
    pub buff: String,
    pub skills: Vec<String>,
}

impl world::WorldObject for Pokemon {
    fn get_id(&self) -> String {
        return self.id.clone();
    }

    fn assign_id(&self, id: String) -> Self {
        Self {
            id: id,
            buff: String::from(&self.buff),
            level: self.level,
            name: String::from(&self.name),
            skills: self.skills.clone(),
        }
    }
}

impl Pokemon {
    pub fn new(name: String, skills: Vec<String>) -> Self {
        Self {
            id: String::from("None"),
            buff: String::from("None"),
            level: 1,
            name: name,
            skills: skills,
        }
    }

    pub fn level_up(&mut self) {
        self.level += 1;
    }
}
