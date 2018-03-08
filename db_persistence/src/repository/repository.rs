pub trait Repository {
  type NewEntity;
  type Entity;

  // TODO Return Result to let implementors be able to return meaningful
  //      errors instead of panicking
  fn save(&self, new_entity: &Self::NewEntity) -> Self::Entity;
}
