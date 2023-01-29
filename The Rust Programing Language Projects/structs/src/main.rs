struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
    fn new(username: String, email: String) -> User {
        User {
            username,
            email,
            active: true,
            sign_in_count: 1,
        }
    }

    fn is_logged_in(&self) -> bool {
        self.active && self.sign_in_count >= 1
    }
}

fn main() {
    //struct Color(i32, i32, i32);
    //struct Point(i32, i32, i32);

    let mut user1 = User {
        email: String::from("nick@mail.com"),
        username: String::from("nick"),
        active: true,
        sign_in_count: 1,
    };

    //let username: String = usclearer1.username;
    user1.username = String::from("nick99");

    let user2 = User {
        email: String::from("alex@mail.com"),
        username: String::from("alex"),
        ..user1
    };

    let user3 = build_user(String::from("tom@gmail.com"), String::from("tom"));
    let user4 = User::new(String::from("tony@mail.com"), String::from("tony"));
    let users = vec![user1, user2, user3, user4];

    for user in users {
        if User::is_logged_in(&user) {
            println!("{} {} is active and has signed in", user.email, user.username);
        }
    }  
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}