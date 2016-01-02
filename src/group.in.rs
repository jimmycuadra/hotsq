use player::Player;

#[derive(Debug, Deserialize, Serialize)]
pub enum Group {
    One(Player),
    Two([Player; 2]),
    Three([Player; 3]),
    Four([Player; 4]),
    Five([Player; 5]),
}

impl Group {
    pub fn len(&self) -> u64 {
        match *self {
            Group::One(_) => 1,
            Group::Two(_) => 2,
            Group::Three(_) => 3,
            Group::Four(_) => 4,
            Group::Five(_) => 5,
        }
    }
}

impl<'a> ::std::fmt::Display for Group {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        match self {
            &Group::One(ref p) => write!(f, "{}", p),
            &Group::Two(ref p) => write!(f, "{}, {}", p[0], p[1]),
            &Group::Three(ref p) => write!(f, "{}, {}, {}", p[0], p[1], p[2]),
            &Group::Four(ref p) => write!(f, "{}, {}, {}, {}", p[0], p[1], p[2], p[3]),
            &Group::Five(ref p) => write!(f, "{}, {}, {}, {}, {}", p[0], p[1], p[2], p[3], p[4]),
        }
    }
}
