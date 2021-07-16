use oop_blog::Post;
use oop_blog::enumpost::Post as EnumPost;
use oop_blog::staticstate::Post as StaticTypePost;

fn main() {
    //----------------------------------------------------------------
    // Dynamic Typing State
    //----------------------------------------------------------------
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

    //----------------------------------------------------------------
    // Enum State
    //----------------------------------------------------------------
    let mut post = EnumPost::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
    
    //----------------------------------------------------------------
    // Static Typing State
    //----------------------------------------------------------------
    let mut post = StaticTypePost::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}
