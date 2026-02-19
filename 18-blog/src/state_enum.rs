use crate::Post;
use crate::State;

pub enum StateImpl {
    Draft,
    PendingReview,
    Published,
}

impl State for StateImpl {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        match *self {
            StateImpl::Draft => Box::new(StateImpl::PendingReview),
            StateImpl::PendingReview => self,
            StateImpl::Published => Box::new(StateImpl::PendingReview),
        }
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        match *self {
            StateImpl::Draft => Box::new(StateImpl::Published),
            StateImpl::PendingReview => Box::new(StateImpl::Published),
            StateImpl::Published => self,
        }
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        match *self {
            StateImpl::Draft => "",
            StateImpl::PendingReview => "",
            StateImpl::Published => &post.content,
        }
    }
}
