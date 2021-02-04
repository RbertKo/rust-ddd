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

fn main() {
  let comment: Comment = Comment {
    author_name: String::from("RbertKo"),
    author_email: String::from("myeongsku@gmail.com"),
    context: String::from("test")
  };
}
