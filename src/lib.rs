
pub struct Post {
  state: Option<Box<dyn State>>,
  content: String,
}

impl Post {
    pub fn new() -> Post{
      Post { state: Some(Box::new(Draft{})), content: String::new() }
    }
    pub fn add_text(&mut self, text: &str) {
      self.content.push_str(text)
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

    pub fn content(&self) -> &str{
      self.state.as_ref().unwrap().content(self)
    }
}

trait State {
  fn request_review(self: Box<Self>) -> Box<dyn State>; 
  fn approve(self: Box<Self>) -> Box<dyn State>;
  fn content<'a>(&self,_post: &'a Post) -> &'a str {
    ""
  }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(Review{})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct Review {}

impl State for Review {
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    self
  }
  fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Publish{})
  }
}

struct Publish {}

impl State for Publish {
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }

}