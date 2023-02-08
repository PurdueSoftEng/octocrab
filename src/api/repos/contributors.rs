use super::*;

#[derive(serde::Serialize)]
pub struct ListContribsBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListContribsBuilder<'octo, 'r> {
    pub fn new(handler: &'r RepoHandler<'octo>, sha: String) -> Self {
        Self {
            handler,
            per_page: None,
            page: None,
        }
    }

    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    pub fn page(mut self, page: impl Into<u32>) -> Self {
        self.page = Some(page.into());
        self
    }

    pub async fn send(self) -> Result<crate::Page<models::repos::Contributor>> {
        let url = format!(
            "repos/{owner}/{repo}/contributors/",
            owner = self.handler.owner,
            repo = self.handler.repo,
        );
        self.handler.crab.get(url, Some(&self)).await
    }
}