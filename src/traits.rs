pub trait CRUD {
  fn list(&self);

  fn detail(&self);

  fn update(&self);

  fn delete(&self);
}
