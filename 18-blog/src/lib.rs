mod state_enum;
mod state_oop;

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn using_oop() -> Post {
        Post {
            state: Some(Box::new(state_oop::Draft {})),
            content: String::new(),
        }
    }

    pub fn using_enum() -> Post {
        Post {
            state: Some(Box::new(state_enum::StateImpl::Draft)),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        self.state
            .as_ref()
            .expect("Post state should not be missing")
            .content(self)
    }

    pub fn request_review(&mut self) {
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

pub trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _post: &'a Post) -> &'a str;
}

#[cfg(test)]
mod tests {
    use super::Post;

    fn no_content_until_approved(mut post: Post) {
        post.add_text("I ate a salad ");
        post.add_text("for lunch today");
        assert_eq!("", post.content());

        post.request_review();
        assert_eq!("", post.content());

        post.approve();
        assert_eq!("I ate a salad for lunch today", post.content());
    }

    #[test]
    fn oop_state_implementation() {
        no_content_until_approved(Post::using_oop());
    }

    #[test]
    fn enum_state_implementation() {
        no_content_until_approved(Post::using_enum());
    }
}
