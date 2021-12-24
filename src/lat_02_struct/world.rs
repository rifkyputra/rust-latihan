
struct World {

}

pub trait WorldObject {
    fn getId(&self)-> String;

    fn assignId(&self, id: String) -> Self;



}

pub trait allObjects {
    fn getAll();
    fn removeById( id: i8);

    fn add<T>(object: T); 
}