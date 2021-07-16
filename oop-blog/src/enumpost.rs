enum PostState {
    Draft,
    PendingReview,
    Published,
}

pub struct Post {
    state: PostState,
    content: String,
    approvals: u32,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: PostState::Draft,
            content: String::new(),
            approvals: 0,
        }
    }

    pub fn add_text(&mut self, text: &str) {
        match self.state {
            PostState::Draft => {
                self.content.push_str(text);
            }
            _ => {
                println!("Adding state is only allowed in Draft state.");
            }
        }
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
            // CAUTIOUS: if we add a new state, we must check all the `_` arms
            // to make sure they are still correct.
            _ => (),
        }
    }

    pub fn approve(&mut self) {
        self.approvals += 1;
        if self.approvals < 2 {
            println!("You got {} approval.", self.approvals);
        } else {
            println!("You got {} approvals. Your post is published!", self.approvals);
        }
        match self.state {
            PostState::PendingReview => {
                if self.approvals == 2 {
                    self.state = PostState::Published;
                }
            }
            _ => (),
        }
    }

    pub fn reject(&mut self) {
        match self.state {
            PostState::PendingReview => {
                self.state = PostState::Draft;
            }
            _ => (),
        }
    }
}

