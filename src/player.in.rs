use hero::HeroName;

#[derive(Debug, Deserialize, Serialize)]
pub struct Player {
    hero_name: HeroName,
    mmr: u64,
}

impl Player {
    pub fn new(hero_name: HeroName, mmr: u64) -> Player {
        Player {
            hero_name: hero_name,
            mmr: mmr,
        }
    }
}
