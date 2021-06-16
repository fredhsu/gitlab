use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Gitlab {
    pub host: String,
    pub project: String,
    pub branch: String,
    pub token: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Commit {
    id: String,
    branch: String,
    commit_message: String,
    actions: Vec<CommitAction>,
}

impl CommitAction {
    pub fn new(action: String, file_path: String, content: String) -> Self {
        CommitAction {
            action,
            file_path,
            content,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CommitAction {
    action: String,
    file_path: String,
    content: String,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
