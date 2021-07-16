pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
            approvals: 0,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
    approvals: u32,
}

impl PendingReviewPost {
    pub fn approve(self) -> PendingReviewPost {
        let new_approvals = self.approvals + 1;
        if new_approvals < 2 {
            println!("You got {} approval.", new_approvals);
        } else {
            println!("You got {} approvals.", new_approvals);
        }

        PendingReviewPost {
            approvals: new_approvals,
            ..self              // `content` is moved, not copied
        }
    }

    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }

    pub fn publish(self) -> Post {
        println!("Your post is published!");
        Post {
            content: self.content,
        }
    }
}
