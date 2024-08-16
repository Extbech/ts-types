use std::fmt::Display;
use ts_types_macros::ts_type;

#[ts_type(export_path = "/frontend/src/")]
pub struct User {
    first_name: String,
    last_name: String,
    age: u32,
}

#[ts_type(export_path = "/backend/src/", file_extension = ".cs")]
pub struct Exercise {
    pub length: f32,
}

impl User {
    pub fn new() -> Self {
        return User {
            first_name: "ching".to_string(),
            last_name: "chong".to_string(),
            age: 25,
        };
    }
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "name: {} {}\nage: {}",
            self.first_name, self.last_name, self.age
        )
    }
}
