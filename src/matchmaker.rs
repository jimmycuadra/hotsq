use plan::Plan;

pub struct Matchmaker<'a> {
    plan: &'a Plan,
}

impl<'a> Matchmaker<'a> {
    pub fn new(plan: &'a Plan) -> Matchmaker<'a> {
        Matchmaker {
            plan: plan,
        }
    }

    pub fn run(&self) {
        println!("Running plan:\n{:?}", self.plan);
    }
}
