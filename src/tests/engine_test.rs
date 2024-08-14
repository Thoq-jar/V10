#[cfg(test)]
mod tests {
    use super::super::engine::Engine;

    #[test]
    fn engine_initializes_correctly() {
        let engine = Engine::new();
        assert_eq!(engine.state, "initialized");
    }
}