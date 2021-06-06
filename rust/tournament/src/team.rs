use std::borrow::Borrow;
use std::hash::{Hash, Hasher};

#[derive(Eq, Clone, Debug, Default)]
pub struct Team {
    /// `name` is used for two purposes.
    /// 1) It is our "key". That is why [`Hash`] is implemented for `Team`
    /// 2) Also so we can [`Borrow`] `Team` as `&str` and do lookup in set.
    ///
    /// [`Hash`]: std::hash::Hash
    /// [`Borrow`]: std::borrow::Borrow
    pub name: String,
    pub matches_played: u64,
    pub matches_won: u64,
    pub matches_drawn: u64,
    pub matches_lost: u64,
    pub points: u64,
}

impl Team {
    pub fn new(team_name: &str) -> Self {
        Team {
            name: String::from(team_name),
            matches_played: 0,
            matches_won: 0,
            matches_drawn: 0,
            matches_lost: 0,
            points: 0,
        }
    }
}
impl Hash for Team {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for Team {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

impl Borrow<str> for Team {
    fn borrow(&self) -> &str {
        self.name.as_str()
    }
}
