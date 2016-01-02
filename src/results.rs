use matchmaker::Matchmaker;

#[derive(Debug)]
pub struct Results<'a> {
    matchmaker: &'a Matchmaker<'a>,
}

impl<'a> Results<'a> {
    pub fn new(matchmaker: &'a mut Matchmaker<'a>) -> Results<'a> {
        Results {
            matchmaker: matchmaker,
        }
    }
}

impl<'a> ::std::fmt::Display for Results<'a> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        let mut output = format!("Game mode:\n{}\n\nInitial queue:\n", self.matchmaker.game_mode());

        for group in self.matchmaker.initial_queue() {
            output.push_str(&format!("{}\n", group));
        }

        output.push_str("\n\nGames:\n");

        for game in self.matchmaker.games() {
            output.push_str(&format!("{}\n", game));
        }

        output.push_str("\n");

        write!(f, "{}", output)
    }
}
