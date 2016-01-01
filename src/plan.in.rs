use rand::random;

use error::Error;
use group::Group;
use mode::Mode;
use player::Player;

#[derive(Debug, Deserialize, Serialize)]
pub struct Plan {
    groups: Vec<Group>,
    mode: Mode,
}

impl Plan {
    pub fn new(mode: Mode, groups: Vec<Group>) -> Result<Plan, Error> {
        try!(mode.validate(&groups));

        Ok(Plan {
            groups: groups,
            mode: mode,
        })
    }

    pub fn generate(mode: Mode, count: u64) -> Result<Plan, Error> {
        let mut groups = vec![];

        for _ in 0..count {
            groups.push(Group::One(Player::new(random(), random())));
        }

        Plan::new(mode, groups)
    }
}
