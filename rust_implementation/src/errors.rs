#[derive(Debug)]
pub struct LoxError {
    pub message: String,
    pub hint: Option<String>,
}