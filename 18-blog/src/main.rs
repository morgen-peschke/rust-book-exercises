use blog::Post;

fn main() {
    let mut post = Post::using_oop();

    post.add_text("I ate a salad for lunch today");
    post.request_review();
    post.approve();
    println!("{}", post.content());
}

