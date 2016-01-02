use group::Group;

#[derive(Debug)]
pub struct Queue<'a> {
    groups: Vec<&'a Group>,
}

impl<'a> Queue<'a> {
    pub fn new(groups: Vec<&'a Group>) -> Queue<'a> {
        Queue {
            groups: groups,
        }
    }

    pub fn player_count(&self) -> u64 {
        self.groups.iter().fold(0, |count, &group| count + group.len())
    }

    pub fn pop(&mut self) -> Option<&'a Group> {
        self.groups.pop()
    }

    pub fn remaining_groups_fit_into_new_game(&self) -> bool {
        false
    }
}
