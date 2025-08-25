use log::info;

use crate::models::Post;
use crate::repository::post_repo::PostRepository;

pub struct PostService {
    post_repository: PostRepository,
}

impl PostService {
    pub fn new(post_repository: PostRepository) -> Self {
        PostService { post_repository }
    }

    pub fn get_all(&self) -> Result<Vec<Post>, String> {
        let posts = self.post_repository.get_all();
        info!("users: {:#?}", posts);
        match posts {
            Ok(posts) => Ok(posts),
            Err(e) => Err(e.to_string()),
        }
    }
}
