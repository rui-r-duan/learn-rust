pub struct Post {
    // Rust doesn't let us have unpopulated fields in structs.
    state: Option<Box<dyn State>>, // a pointer to a NULL-able object
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        let curr_state = self.state.as_ref().unwrap();
        let can_add_text = curr_state.can_add_text();
        if can_add_text {
            self.content.push_str(text);
        } else {
            println!("Adding state is only allowed in Draft state.");
        }
    }

    pub fn content(&self) -> &str {
        // as_ref() converts from &Option<T> to Option<&T>.
        //
        // as_ref() returns an `Option<&Box<dyn State>>`.  Then deref coercion
        // will take effect on the `&` and the `Box` so the `content` method
        // will ultimately be called on the type that implements the `State`
        // trait.
        //
        // What happens without calling `as_ref()`?
        //
        // error[E0507]: cannot move out of `self.state` which is behind a shared reference
        //   --> src/lib.rs:26:9
        //    |
        // 26 |         self.state.unwrap().content(self)
        //    |         ^^^^^^^^^^
        //    |         |
        //    |         move occurs because `self.state` has type `Option<Box<dyn State>>`, which does not implement the `Copy` trait
        //    |         help: consider borrowing the `Option`'s content: `self.state.as_ref()`
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        // If we did not use `Option<>` as `state` value, what is going to happen to
        // ``` self.state = self.state.request_review() ```?
        //
        // error[E0507]: cannot move out of `self.state` which is behind a mutable reference
        //    --> src/lib.rs:30:22
        //     |
        //  30 |         self.state = self.state.request_review();
        //     |                      ^^^^^^^^^^ move occurs because `self.state` has type `Box<dyn State>`, which does not implement the `Copy` trait
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }
}

trait State {
    // Rather than having `self`, `&self`, or `&mut self` as the first
    // parameter of the method, `self: Box<Self>` is used.  This syntax means
    // the method is only valid when called on a `Box` holding the type.  This
    // syntax takes ownership of Box<Self>, invalidating the old state so the
    // state value of the `Post` can transform into a new state.
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;

    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }

    fn can_add_text(&self) -> bool {
        false
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview { approvals: 0 })
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn can_add_text(&self) -> bool {
        true
    }
}

struct PendingReview {
    approvals: u32,
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        let new_approvals = self.approvals + 1;
        if new_approvals < 2 {
            println!("You got {} approval!", new_approvals);
            Box::new(PendingReview { approvals: new_approvals })
        } else {
            println!("You got {} approvals! Your post is published!", new_approvals);
            Box::new(Published {})
        }
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

pub mod enumpost;
pub mod staticstate;
