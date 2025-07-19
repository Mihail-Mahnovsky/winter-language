#[derive(Debug, Clone)]
pub struct numberNode {
    value: i32,
}

impl numberNode {
    pub fn new(value: i32) -> Self {
        Self { value }
    }

    pub fn get_value(&self) -> i32 {
        return self.value;
    }
}
