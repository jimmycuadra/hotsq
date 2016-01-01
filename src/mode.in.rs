use error::Error;
use group::Group;

#[derive(Debug, Deserialize, Serialize)]
pub enum Mode {
    HeroLeague,
    QuickMatch,
    TeamLeague,
}

impl Mode {
    pub fn validate<'a>(&self, groups: &'a [Group]) -> Result<(), Error> {
        match *self {
            Mode::HeroLeague => {
                for group in groups.iter() {
                    match *group {
                        Group::One(_) | Group::Two(_) | Group::Three(_) => {},
                        _ => return Err(Error::new("Only groups of 2-4 are allowed in Hero League")),
                    }
                }
            },
            Mode::QuickMatch => {},
            Mode::TeamLeague => {
                for group in groups.iter() {
                    match *group {
                        Group::Five(_) => {},
                        _ => return Err(Error::new("Only groups of 5 are allowed in Team League")),
                    }
                }
            },
        }

        Ok(())
    }
}
