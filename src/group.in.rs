use player::Player;

#[derive(Debug, Deserialize, Serialize)]
pub enum Group {
    One(Player),
    Two([Player; 2]),
    Three([Player; 3]),
    Four([Player; 4]),
    Five([Player; 5]),
}
