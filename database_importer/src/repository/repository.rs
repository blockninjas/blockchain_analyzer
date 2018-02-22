pub trait Repository<T> {

    fn save(&self, entity: &T);
}
