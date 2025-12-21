use std::sync::{Arc, Mutex};

use crate::models::user::User;

pub type AppState = Arc<Mutex<Vec<User>>>;
