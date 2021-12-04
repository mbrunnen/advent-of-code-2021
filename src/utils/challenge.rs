pub trait Challenge {
    fn new(lines: Vec<String>) -> Self;
    fn run(&self) -> Result<String, String>;
}
