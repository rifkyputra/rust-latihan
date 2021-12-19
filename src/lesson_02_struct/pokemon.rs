pub struct Pokemon {
    pub name: String,
    pub level: i8,
    pub buff: String,
    pub skills: Vec<String>,

}

pub struct Creep {
    level: i8,
    buff: String,
}

impl Pokemon {
    pub fn level_up(&mut self) {
        self.level += 1;

    }
}


