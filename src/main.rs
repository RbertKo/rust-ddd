struct Post {
  context: String,
  comment: Vec<Comment>,
  author_name: String,
  author_email: String,
}

struct Comment {
  author_name: String,
  author_email: String,
  context: String,
}

impl Post {
  fn add_comment(comment) {

  } 
}

fn main() {
  let post: Post = Post {
    context: String::from("이것은 테스트를 위한 글입니다."),
    comment: vec![],
    author_name: String::from("fakeRbertKo"),
    author_email: String::from("fake_rbertko@gmail.com")
  }; 

  let comment: Comment = Comment {
    author_name: String::from("RbertKo"),
    author_email: String::from("myeongsku@gmail.com"),
    context: String::from("test")
  };
}