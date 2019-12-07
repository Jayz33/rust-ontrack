fn main() {
    // ==== Memory management & ownership ====
    // ============ Challenge 2.1 ============
    // fn greeting(name: String) {
    //     println!("Hello, {}", name);
    // }
    
    // fn print_length(s: String) {
    //     println!("{} is {} bytes long", s, s.len());
    // }

    // let name = String::from("Bob");
    // print_length(name);
    // greeting(name); // error: use of moved value: `name`

    // ============ Challenge 2.2 ============
    // fn greeting(name: String) {
    //     println!("Hello, {}", name);
    // }
    
    // fn print_length(s: String) {
    //     println!("{} is {} bytes long", s, s.len());
    // }

    // let name = String::from("Bob");
    // print_length(name.clone());
    // greeting(name.clone());

    // ============ Challenge 2.3 ============
    // fn greeting(name: &String) {
    //     println!("Hello, {}", name);
    // }
    
    // fn print_length(s: &String) {
    //     println!("{} is {} bytes long", s, s.len());
    // }

    // let name = String::from("Bob");
    // print_length(&name);
    // greeting(&name);

    // ============ Challenge 2.4 ============
    fn greeting(name: &str) {
        println!("Hello, {}", name);
    }
    
    fn print_length(s: &str) {
        println!("{} is {} bytes long", s, s.len());
    }

    let name = "Bob";
    print_length(name);
    greeting(name);

    // ===== Object Oriented Programming =====
    // ============= Challenge 3 =============
    struct Player {
        name: String,
        level: u32
    }

    impl Player {
        pub fn new(name: String, level: u32) -> Player {
            Player {
                name,
                level
            }
        }

        pub fn level_up(&mut self) {
            self.level += 1;
            println!("Player {} has advanced to level {}", self.name, self.level);
        }
    }

    let mut player = Player::new(String::from("Bob"), 0);
    player.level_up();
    player.level_up();

    // ====== Enums & pattern matching =====
    // ============ Challenge 4 ============
    enum Action {
        Move { x: i32, y: i32 },
        Speak(String),
        AddLevels(u32)
    }

    impl Player {
        pub fn handle_action(&mut self, action: &Action) {
            match action {
                Action::Move { x, y } => println!("Player {} is moving to x: {}, y: {}", self.name, x, y),
                Action::Speak(s) => println!("Player {} says: {}", self.name, s),
                Action::AddLevels(l) => {
                    self.level += l;
                    println!("Player {} is now level {}", self.name, self.level);
                }
            }
        }
    }

    let move_action = Action::Move { x: 1, y: 2 };
    player.handle_action(&move_action);

    let speak_action = Action::Speak(String::from("Hello"));
    player.handle_action(&speak_action);

    let add_levels_action = Action::AddLevels(20);
    player.handle_action(&add_levels_action);

    // ==== Generics, traits & lifetimes ===
    // ============ Challenge 5 ============
    impl Player {
        pub fn get_level(&self) -> u32 {
            self.level
        }
    }

    // fn player_with_highest_level(player1: &Player, player2: &Player) -> &Player { // error: missing lifetime specifier
    //     if player1.get_level() > player2.get_level() {
    //         player1 // error: explicit lifetime required in the type of `player1`
    //     } else {
    //         player2 // error: explicit lifetime required in the type of `player2`
    //     }
    // }

    fn player_with_highest_level<'p>(player1: &'p Player, player2: &'p Player) -> &'p Player {
        if player1.get_level() > player2.get_level() {
            player1
        } else {
            player2
        }
    }

    let other_player = Player::new(String::from("Alice"), 10);
    let best_player = player_with_highest_level(&player, &other_player);
    println!("The best player is {} (level {})", best_player.name, best_player.get_level());
}

