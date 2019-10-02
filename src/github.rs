use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    pub contributions: ContributionCollection,
    pub repositories: HashMap<String, i64>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ContributionCollection {
    #[serde(rename = "totalContributions")]
    pub total_contributions: i64,
    pub colors: Vec<String>,
    pub weeks: HashMap<usize, Week>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Week {
    pub days: HashMap<usize, Contribution>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Contribution {
    pub contribution_count: i64,
    pub color: String,
}

impl User {
    pub fn new() -> Self {
        User {
            contributions: ContributionCollection::new(),
            repositories: HashMap::new(),
        }
    }
}

impl ContributionCollection {
    pub fn new() -> Self {
        ContributionCollection {
            total_contributions: 0,
            colors: Vec::new(),
            weeks: HashMap::new(),
        }
    }
}

impl Week {
    pub fn new() -> Self {
        Week {
            days: HashMap::new(),
        }
    }
}

impl Contribution {
    pub fn new(contribution_count: i64, color: String) -> Self {
        Contribution {
            contribution_count,
            color,
        }
    }
}
