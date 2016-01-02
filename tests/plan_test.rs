extern crate hotsq;

use hotsq::{Matchmaker, Mode, Plan};

#[test]
fn random_plan() {
    let plan = Plan::generate(Mode::QuickMatch, 100).expect("failed to generate plan");
    let matchmaker = Matchmaker::new(&plan);
    matchmaker.run();
}
