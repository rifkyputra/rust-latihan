use crate::world;

pub struct Creep {
    level: i8,
    buff: String,
    id: String,
}

impl world::WorldObject for Creep {

    fn getId(&self) -> String{
        return self.id.clone();
    }

    fn assignId(&self, id: String) -> Self {
        Self {
            id: id,
            buff : String::from(&self.buff),
            level : self.level,
        }
    }

}


impl Creep {
    pub fn new(level: i8, buff: String) -> Self {
        Self {
            id: String::from("None"),
            level: level,
            buff: buff,
        }
    }
}
