pub struct Post {
    // Rust doesn't let us have unpopulated fields in structs.
    // Can we use a mut variable in structs?
    state: Option<Box<dyn State>>, // a pointer to a NULL-able object
    content: String,
}

impl Post {
    // --snip--
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
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
}

trait State {
    // Rather than having `self`, `&self`, or `&mut self` as the first
    // parameter of the method, `self: Box<Self>` is used.  This syntax means
    // the method is only valid when called on a `Box` holding the type.  This
    // syntax takes ownership of Box<Self>, invalidating the old state so the
    // state value of the `Post` can transform into a new state.
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
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

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
