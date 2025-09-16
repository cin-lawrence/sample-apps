use crate::models::{Comment, Page};

pub struct HackerNews {}

pub static BASE_API_URL: &str = "https://hacker-news.firebaseio.com/v0/";
pub static ITEM_API: &str = "item/";
pub static USER_API: &str = "user/";
const COMMENT_DEPTH: i64 = 1;

impl HackerNews {
    pub async fn get_top_stories() -> reqwest::Result<Vec<i64>> {
        let url = format!("{}topstories.json", BASE_API_URL);
        let mut stories_ids = reqwest::get(&url).await?.json::<Vec<i64>>().await?;
        stories_ids.truncate(30);
        Ok(stories_ids)
    }

    pub async fn get_comment(comment: i64) -> reqwest::Result<Comment> {
        let url = format!("{}{}{}.json", BASE_API_URL, ITEM_API, comment);
        reqwest::get(&url).await?.json::<Comment>().await
    }

    pub async fn get_page(id: i64) -> reqwest::Result<Page> {
        let url = format!("{}{}{}.json", BASE_API_URL, ITEM_API, id);
        reqwest::get(&url).await?.json::<Page>().await
    }
}
