struct Post {
  user: User,
  context: String,
  comments: Vec<Comment>,
}

struct Comment {
  user: User,
  context: String,
}

struct User {
  name: String,
  email: String,
}

impl Post {
  fn add_comment(&mut self, comment: Comment) {
    self.comments.push(comment);
  } 
}

fn main() {
  let user1: User = User {
    name: String::from("fakeRbertKo"),
    email: String::from("fake_rbertko@gmail.com")
  };

  let user2: User = User {
    name: String::from("RbertKo"),
    email: String::from("myeongsku@gmail.com"),
  };

  let mut post: Post = Post {
    user: user1,
    context: String::from("이것은 테스트를 위한 글입니다."),
    comments: vec![],
  }; 

  let comment: Comment = Comment {
    user: user2,
    context: String::from("test")
  };

  post.add_comment(comment);
}