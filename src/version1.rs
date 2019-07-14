use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt;

pub const PROTOCOL_VERSION: &str = "1.0";

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Stop {
    pub id: String,
    pub title: String,
    pub body: String,
    pub rel_path: String,
    pub repository: String,
    pub line: usize,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Repository {
    pub repository: String,
    pub commit: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TourFile {
    pub protocol_version: String,
    pub id: String,
    pub title: String,
    pub description: String,
    pub stops: Vec<Stop>,
    pub repositories: Vec<Repository>,
}

impl TryFrom<&str> for TourFile {
    type Error = serde_json::Error;
    fn try_from(tf: &str) -> Result<TourFile, Self::Error> {
        serde_json::from_str(tf)
    }
}

impl fmt::Display for TourFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(self).or(Err(fmt::Error))?)
    }
}

impl TourFile {
    pub fn to_tour(&self) -> tourist_types::Tour {
        tourist_types::Tour {
            protocol_version: self.protocol_version.to_owned(),
            id: self.id.to_owned(),
            title: self.title.to_owned(),
            description: self.description.to_owned(),
            stops: self
                .stops
                .iter()
                .map(|stop| tourist_types::Stop {
                    id: stop.id.to_owned(),
                    title: stop.title.to_owned(),
                    body: stop.body.to_owned(),
                    path: stop.rel_path.as_str().into(),
                    repository: stop.repository.to_owned(),
                    line: stop.line,
                })
                .collect::<Vec<_>>(),
            repositories: self
                .repositories
                .iter()
                .map(|r| (r.repository.to_owned(), r.commit.to_owned()))
                .collect::<HashMap<_, _>>(),
        }
    }
}
