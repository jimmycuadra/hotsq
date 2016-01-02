use game::{FULL_GAME_SIZE, Game};
use group::Group;
use mode::Mode;
use plan::Plan;
use queue::Queue;
use results::Results;

#[derive(Debug)]
pub struct Matchmaker<'a> {
    games: Vec<Game<'a>>,
    plan: &'a Plan,
    queue: Queue<'a>,
}

impl<'a> Matchmaker<'a> {
    pub fn new(plan: &'a Plan) -> Matchmaker<'a> {
        Matchmaker {
            games: vec![],
            plan: plan,
            queue: Queue::new(plan.groups_vec()),
        }
    }

    pub fn run(&'a mut self) -> Results<'a> {
        let mut game = Game::new();

        loop {
            let remaining_players = self.queue.player_count();

            if remaining_players == 0 {
                break;
            }

            if game.is_filled() {
                self.games.push(game);

                if remaining_players < FULL_GAME_SIZE {
                    break;
                } else if !self.queue.remaining_groups_fit_into_new_game() {
                    break;
                } else {
                    game = Game::new();
                }
            } else {
                game.add_group(self.queue.pop().expect("no remaining groups to place"));
            }
        }

        Results::new(self)
    }

    pub fn game_mode(&self) -> Mode {
        self.plan.mode()
    }

    pub fn games(&'a self) -> &'a Vec<Game<'a>> {
        &self.games
    }

    pub fn initial_queue(&self) -> Vec<&Group> {
        self.plan.groups_vec()
    }
}

#[cfg(test)]
mod tests {
    use mode::Mode;
    use super::Matchmaker;
    use plan::Plan;

    #[test]
    fn random_qm() {
        let plan = Plan::generate(Mode::QuickMatch, 100).expect("failed to generate plan");
        let mut matchmaker = Matchmaker::new(&plan);
        let results = matchmaker.run();
        println!("{}", results);
    }
}
