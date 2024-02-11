extern crate oops; 
use oops::Post;

fn main() {
    let mut post = Post::new(); 
    post.add_text("Hello world"); 
    assert_eq!("", post.content()); 

    post.request_review(); 
    assert_eq!("", post.content()); 

    post.approve(); 
    assert_eq!("Hello world", post.content()); 

}



