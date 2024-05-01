
pub struct User {
    pub name: String,
    pub email: String,
    pub age: u8,
}

impl User {
    pub fn get_name(&self) -> &str {
        return &self.name;
    }

    pub fn get_email(&self) -> &str {
        return &self.email;
    }

    pub fn get_age(&self) -> u8 {
        return self.age;
    }
}

pub fn new_user(name: String, email: String, age: String) -> User {
    let user = User{name: name, email: email, 
        age: match age.trim().parse().ok() {
                Some(val) => val,
                None => 0
            }
        };
    return user;
}