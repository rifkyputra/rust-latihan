pub trait WorldObject {
    fn get_id(&self) -> String;

    fn assign_id(&self, id: String) -> Self;
}

pub trait AllObjects {
    fn get_all();
    fn remove_by_id(id: i8);

    fn add<T>(object: T);
}
