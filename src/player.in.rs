use hero::HeroName;

#[derive(Debug, Deserialize, Serialize)]
pub struct Player {
    hero_name: HeroName,
}

impl Player {
    pub fn new(hero_name: HeroName) -> Player {
        Player {
            hero_name: hero_name,
        }
    }
}

impl<'a> ::std::fmt::Display for Player {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        write!(f, "{}", self.hero_name)
    }
}
