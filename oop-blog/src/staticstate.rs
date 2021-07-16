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
            required_approvals: 2,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
    approvals: u32,
    required_approvals: u32,
}

impl PendingReviewPost {
    pub fn approve(&mut self) {
        let new_approvals = self.approvals + 1;
        if new_approvals < self.required_approvals {
            println!("You got {} approval.", new_approvals);
        } else {
            println!("You got {} approvals.", new_approvals);
        }

        self.approvals = new_approvals;
    }

    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }

    pub fn allow_publish(&self) -> bool {
        self.approvals >= self.required_approvals
    }

    pub fn publish(self) -> Post {
        // This assertion is used to warn developers for the violation of the contract.
        assert!(self.allow_publish());
        println!("Your post is published!");
        Post {
            content: self.content,
        }
    }
}
