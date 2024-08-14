#[cfg(test)]
mod tests {
  use crate::engine::engine::Engine;

  #[test]
  fn engine_initializes_correctly() {
    let engine: Engine = Engine::new();
    assert_eq!(engine.state, "initialized");
  }
}
