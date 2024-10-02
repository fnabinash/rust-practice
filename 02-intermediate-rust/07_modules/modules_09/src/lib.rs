// Write integration tests for your library crate in the tests directory.

pub struct User {
    username: String,
    password: String
}

impl User {
    pub fn default_user() -> Self {
        User {
            username: "Sam".to_string(),
            password: "sam".to_string()
        }
    }

    pub fn change_username(&mut self, password: &str, new_username: &str) -> Result<(), String>{
        let old_username: String = self.username.clone();

        if self.password == password {
            if self.username == new_username {
                Err("Error: Please enter a new username.".to_string())
            } else {
                self.username = new_username.to_string();
                println!("Username changed from {:?} to {:?}.", old_username, new_username);
                Ok(())
            }
        } else {
            Err("Error: Wrong password!".to_string())
        }
    }

    pub fn change_password(&mut self, password: &str, new_password: &str) -> Result<(), String>{
        let old_password: String = self.password.clone();

        if old_password == password {
            if old_password == new_password {
                Err("Error: Please enter a new password.".to_string())
            } else {
                self.password = new_password.to_string();
                Ok(())
            }
        } else {
            Err("Error: Wrong password!".to_string())
        }
    }

    pub fn username(&self) -> String {
        self.username.to_string()
    }
}
