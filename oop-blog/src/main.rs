use oop_blog::Post;
use oop_blog::enumpost::Post as EnumPost;
use oop_blog::staticstate::Post as StaticTypePost;

fn main() {
    //----------------------------------------------------------------
    // Dynamic Typing State
    //----------------------------------------------------------------
    println!("-------- Dynamic Typing State --------");
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today.");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.add_text(" This is a nice day.");
    assert_eq!("", post.content());

    post.reject();
    assert_eq!("", post.content());

    // After being rejected, if we do not call `request_review()`,
    // `approve()` will panic!
    post.request_review();

    post.approve();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today.", post.content());

    //----------------------------------------------------------------
    // Enum State
    //----------------------------------------------------------------
    println!("-------- Enum State --------");
    let mut post = EnumPost::new();

    post.add_text("I ate a salad for lunch today.");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.add_text(" This is a nice day.");
    assert_eq!("", post.content());

    post.reject();
    assert_eq!("", post.content());

    // After being rejected, if we do not call `request_review()`,
    // `approve()` will panic!
    post.request_review();

    post.approve();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today.", post.content());

    //----------------------------------------------------------------
    // Static Typing State
    //----------------------------------------------------------------
    println!("-------- Static Typing State --------");
    let mut post = StaticTypePost::new();

    post.add_text("I ate a salad for lunch today.");

    let post = post.request_review();

    // Won't compile!  Because method not found in `PendingReviewPost`.
    // Only `Draft` state has `add_text`.  It has met our new requirement.
    // post.add_text("I ate a salad for lunch today.");

    let post = post.reject();

    // If we call `approve()` directly after `reject()`, it won't compile!
    // It means that the static type system can enforce the state change rules!
    let post = post.request_review();

    let post = post.approve();
    let post = post.approve();

    // CAUTIOUS: it is not easy for the static typing solution to use `approve()`
    // to return two types of state, so we are forced to add another user API
    // `publish`.
    let post = post.publish();

    assert_eq!("I ate a salad for lunch today.", post.content());
}
