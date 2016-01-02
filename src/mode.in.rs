use error::Error;
use group::Group;

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
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

    pub fn as_str(&self) -> &'static str {
        match *self {
            Mode::HeroLeague => "Hero League",
            Mode::QuickMatch => "Quick Match",
            Mode::TeamLeague => "Team League",
        }
    }
}

impl<'a> ::std::fmt::Display for Mode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        write!(f, "{}", self.as_str())
    }
}
