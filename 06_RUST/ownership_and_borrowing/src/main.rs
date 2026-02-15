use std::cell::RefCell;

struct ProfileInfo {
    username: String,
    team: String,
}

struct PlayerStats {
    name: String,
    level: u32,
    profile: ProfileInfo,
}

struct Player {
    data: RefCell<PlayerStats>,
}

impl Player {
    fn new(name: String, level: u32, username: String, team: String) -> Self {
        Player {
            data: RefCell::new(PlayerStats {
                name,
                level,
                profile: ProfileInfo { username, team },
            }),
        }
    }

    fn set_name(&self, name: String) {
        self.data.borrow_mut().name = name;
        self.print_state();
    }

    fn set_level(&self, level: u32) {
        self.data.borrow_mut().level = level;
        self.print_state();
    }

    fn set_username(&self, username: String) {
        self.data.borrow_mut().profile.username = username;
        self.print_state();
    }

    fn set_team(&self, team: String) {
        self.data.borrow_mut().profile.team = team;
        self.print_state();
    }

    fn update_all(&self, name: String, level: u32, username: String, team: String) {
        *self.data.borrow_mut() = PlayerStats {
            name,
            level,
            profile: ProfileInfo { username, team },
        };
        self.print_state();
    }

    fn print_state(&self) {
        let data = self.data.borrow();

        println!("\n\nusing first reference\n");
        println!("Name: {}", data.name);
        println!("Level: {}", data.level);
        println!("Username: {}", data.profile.username);
        println!("Team: {}", data.profile.team);


        println!("\n\nusing second reference\n");
        println!("Name: {}", data.name);
        println!("Level: {}", data.level);
        println!("Username: {}", data.profile.username);
        println!("Team: {}", data.profile.team);
      
    }
}

fn main() {
    let player1 = Player::new(
        "Shlok".to_string(),
        10,
        "shlok123".to_string(),
        "coders".to_string(),
    );

    println!("Initial State:");
    player1.print_state();

    let player2 = &player1;

    player2.set_name("Shlok34".to_string());
    player2.set_level(15);
    player2.set_username("newName".to_string());
    player2.set_team("hello".to_string());

    println!("Final State (accessed through player2):");
    player2.print_state();

    println!("Using update_all method:");
    player2.update_all(
        "name2".to_string(),
        25,
        "shlok89".to_string(),
        "newTeam".to_string(),
    );
}
