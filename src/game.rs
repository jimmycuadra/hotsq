use group::Group;

pub const FULL_GROUP_SIZE: u64 = 5;
pub const FULL_GAME_SIZE: u64 = FULL_GROUP_SIZE * 2;

#[derive(Debug)]
pub struct Game<'a> {
    blue: Vec<&'a Group>,
    red: Vec<&'a Group>,
}

impl<'a> Game<'a> {
    pub fn new() -> Game<'a> {
        Game {
            blue: vec![],
            red: vec![],
        }
    }

    pub fn is_filled(&self) -> bool {
        self.is_blue_filled() && self.is_red_filled()
    }

    pub fn add_group(&mut self, group: &'a Group) {
        if self.is_blue_filled() {
            self.red.push(group);
        } else {
            self.blue.push(group);
        }
    }

    fn is_blue_filled(&self) -> bool {
        self.blue_player_count() >= FULL_GROUP_SIZE
    }

    fn is_red_filled(&self) -> bool {
        self.red_player_count() >= FULL_GROUP_SIZE
    }

    fn blue_player_count(&self) -> u64 {
        self.blue.iter().fold(0, |count, &group| count + group.len() as u64)
    }

    fn red_player_count(&self) -> u64 {
        self.red.iter().fold(0, |count, &group| count + group.len() as u64)
    }
}

impl<'a> ::std::fmt::Display for Game<'a> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        let mut output = format!("- Blue: ");

        output.push_str(&join(self.blue.iter().map(|group| {
            format!("{}", group)
        }).collect(), ", "));

        output.push_str("\n  Red: ");

        output.push_str(&join(self.red.iter().map(|group| {
            format!("{}", group)
        }).collect(), ", "));

        write!(f, "{}", output)
    }
}

fn join<S:ToString> (l: Vec<S>, sep: &str) -> String{
    l.iter().fold("".to_string(), |a, b| if a.len() > 0 { a + sep } else { a } + &b.to_string())
}
