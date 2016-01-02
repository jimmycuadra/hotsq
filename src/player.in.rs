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
