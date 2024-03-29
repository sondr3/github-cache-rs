use crate::query::GithubResponse;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type Repositories = HashMap<String, i64>;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    pub contributions: ContributionCollection,
    pub repositories: Repositories,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ContributionCollection {
    #[serde(rename = "totalContributions")]
    pub total_contributions: i64,
    pub colors: Vec<String>,
    pub weeks: HashMap<usize, Week>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Week {
    pub days: HashMap<usize, Contribution>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Contribution {
    pub contribution_count: i64,
    pub color: String,
    pub date: String,
}

impl User {
    fn new() -> Self {
        User {
            contributions: ContributionCollection::new(),
            repositories: HashMap::new(),
        }
    }

    pub fn from_response(response: GithubResponse) -> Self {
        let mut user = User::new();

        user.contributions.total_contributions = response.contributions.total_contributions;
        for color in &mut response.contributions.colors.into_iter() {
            user.contributions.colors.push(color);
        }

        for (i, week) in &mut response.contributions.weeks.into_iter().enumerate() {
            let mut w = Week::new();

            for (d, contrib) in week.contribution_days.iter().enumerate() {
                w.days.insert(
                    d,
                    Contribution::new(
                        contrib.contribution_count,
                        contrib.color.to_owned(),
                        contrib.date.to_owned(),
                    ),
                );
            }
            user.contributions.weeks.insert(i, w);
        }

        for node in &mut response
            .repositories
            .nodes
            .as_ref()
            .expect("Missing repository nodes")
            .iter()
        {
            if let Some(node) = node {
                user.repositories
                    .insert(node.name.to_owned(), node.stargazers.total_count);
            }
        }

        user
    }
}

impl ContributionCollection {
    fn new() -> Self {
        ContributionCollection {
            total_contributions: 0,
            colors: Vec::new(),
            weeks: HashMap::new(),
        }
    }
}

impl Week {
    fn new() -> Self {
        Week {
            days: HashMap::new(),
        }
    }
}

impl Contribution {
    fn new(contribution_count: i64, color: String, date: String) -> Self {
        Contribution {
            contribution_count,
            color,
            date,
        }
    }
}
