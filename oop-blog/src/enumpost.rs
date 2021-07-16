enum PostState {
    Draft,
    PendingReview,
    Published,
}

pub struct Post {
    state: PostState,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: PostState::Draft,
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        match self.state {
            PostState::Draft | PostState::PendingReview => "",
            PostState::Published => &self.content,
        }
    }

    pub fn request_review(&mut self) {
        match self.state {
            PostState::Draft => {
                self.state = PostState::PendingReview;
            }
            _ => (),
        }
    }

    pub fn approve(&mut self) {
        match self.state {
            PostState::PendingReview => {
                self.state = PostState::Published;
            }
            _ => (),
        }
    }
}

