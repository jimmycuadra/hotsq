use plan::Plan;

#[derive(Debug)]
pub struct Matchmaker<'a> {
    plan: &'a Plan,
}

impl<'a> Matchmaker<'a> {
    pub fn new(plan: &'a Plan) -> Matchmaker<'a> {
        Matchmaker {
            plan: plan,
        }
    }

    pub fn run(&self) -> Results<'a> {
        Results {
            plan: self.plan,
        }
    }
}

#[derive(Debug)]
pub struct Results<'a> {
    plan: &'a Plan,
}

#[cfg(test)]
mod tests {
    use mode::Mode;
    use super::Matchmaker;
    use plan::Plan;

    #[test]
    fn random_qm() {
        let plan = Plan::generate(Mode::QuickMatch, 100).expect("failed to generate plan");
        let matchmaker = Matchmaker::new(&plan);
        let results = matchmaker.run();
        println!("{:?}", results);
    }
}
