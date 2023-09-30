use std::env;

pub struct Passwords {
    one: String,
    two: String,
    three: String,
}

impl Passwords {
    fn to_list(&self) -> Vec<&String> {
        [&self.one, &self.two, &self.three].to_vec()
    }

    pub fn check_password(&self, input_password: &String) -> bool {
        self.to_list().contains(&input_password)
    }

    pub fn load_passwords_from_env() -> Self {
        let p1 = env::var("PASSWORD_ONE").unwrap();
        let p2 = env::var("PASSWORD_TWO").unwrap();
        let p3 = env::var("PASSWORD_THREE").unwrap();

        Self {
            one: p1,
            two: p2,
            three: p3,
        }
    }
}
