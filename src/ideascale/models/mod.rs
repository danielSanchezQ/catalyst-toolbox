use std::collections::HashMap;

pub mod custom_fields;
pub mod de;
pub mod se;

pub type Scores = HashMap<u32, f32>;
