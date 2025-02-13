use std::fmt;

use crate::Repo;

#[derive(Default)]
pub struct Query {
    repo: Vec<String>,
    is: Vec<String>,
    label: Vec<String>,
    r#type: Vec<String>,
    state: Vec<String>,
    no: Vec<String>,
}

impl Query {
    pub fn new() -> Self {
        Query {
            ..Default::default()
        }
    }

    pub fn from_repo(repo: Repo) -> Self {
        let repo = vec![String::from(repo.full_name())];
        Query {
            repo,
            ..Default::default()
        }
    }

    /// *Adds* a repo to the query.
    ///
    /// Results in `repo:user/repo`.
    pub fn repo(mut self, user: &str, repo: &str) -> Self {
        self.repo.push(format!("{}/{}", user, repo));
        self
    }

    /// *Adds* an `is` statement to the query.
    ///
    /// Results in `is:statement`.
    pub fn is(mut self, statement: &str) -> Self {
        self.is.push(String::from(statement));
        self
    }

    /// *Adds* a `label` statement to the query.
    ///
    /// Results in `label:statement`.
    pub fn label(mut self, statement: &str) -> Self {
        self.label.push(String::from(statement));
        self
    }

    /// *Adds* a `type` statement to the query.
    ///
    /// Results in `type:statement`.
    ///
    /// *Use `r#type` to escape `type` keyword.
    pub fn r#type(mut self, statement: &str) -> Self {
        self.r#type.push(String::from(statement));
        self
    }

    /// *Adds* a `no` statement to the query.
    ///
    /// Results in `no:statement`.
    pub fn no(mut self, statement: &str) -> Self {
        self.no.push(String::from(statement));
        self
    }
}

impl fmt::Display for Query {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let queries = {
            let mut repo: Vec<String> = self.repo.iter().map(|s| format!("repo:{}", s)).collect();
            let mut is: Vec<String> = self.is.iter().map(|s| format!("is:{}", s)).collect();
            let mut label: Vec<String> = self.label.iter().map(|s| format!("label:{}", s)).collect();
            let mut r#type: Vec<String> =
                self.r#type.iter().map(|s| format!("type:{}", s)).collect();
            let mut state: Vec<String> =
                self.state.iter().map(|s| format!("state:{}", s)).collect();
            let mut no: Vec<String> =
                self.no.iter().map(|s| format!("no:{}", s)).collect();

            let mut queries: Vec<String> =
                Vec::with_capacity(repo.len() + is.len() + label.len() + r#type.len() + state.len()
                    + no.len());

            queries.append(&mut repo);
            queries.append(&mut is);
            queries.append(&mut label);
            queries.append(&mut r#type);
            queries.append(&mut state);
            queries.append(&mut no);
            queries
        };

        let queries = queries.join("+");

        write!(f, "q={}", queries)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn built_query() {
        let query = Query::new()
            .repo("rust-lang", "rust")
            .r#type("pr")
            .is("merged")
            .label("hacktoberfest")
            .no("assignee")
            .to_string();

        assert_eq!("q=repo:rust-lang/rust+is:merged+label:hacktoberfest+type:pr+no:assignee", query);
    }
}
