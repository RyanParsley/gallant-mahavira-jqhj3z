use crate::{author::Author, post::Post, response::Response};
use std::fs;
mod author;
mod post;
mod response;

/*

Assuming a backend service returns data to you in the form of source,
write a couple functions to make the unit tests pass.

Note: `source` groups posts by category (diy and cooking). When you're
no longer grouping in this way, be sure to not lose this context
(prescribed solution is in the test data).

*/

// Write some code already!
fn main() {
    // This may come in handy, don't feel obligated to use it.
    let _authors: Vec<Author> = vec![
        {
            Author {
                id: 1,
                name: "Ryan".to_string(),
                bio: "I like to make things".to_string(),
            }
        },
        {
            Author {
                id: 2,
                name: "Jon".to_string(),
                bio: "I too like to make things".to_string(),
            }
        },
    ];

    let source: Response = serde_json::from_str(
        &fs::read_to_string("./src/response.json").expect("Uh oh! I can\'t open the file."),
    )
    .unwrap();

    let _all = get_all_posts(&source);

    let _first = get_post_by_id(&source, 1);

    let _by_author = get_posts_by_author_id(&source, 0);

    let _by_category = get_posts_by_category(&source, "diy");


    println!("The response is {:?}", source);

    println!("by author {:?}", source);
}

fn get_all_posts(response: &Response) -> Vec<Post> {
    // todo: this works, but doesn't scale
    // enhance to work for any number of categories
    [
        &response.cooking.posts[..],
        &response.diy.posts[..]
    ].concat()
}

fn get_post_by_id(collection: &Response, id: i64) -> Post {
    get_all_posts(collection).into_iter().filter(|post: &Post| post.id == id).collect::<Vec<_>>()[0].clone()
}

fn get_posts_by_author_id(collection: &Response, id: i64) -> Vec<Post> {
    get_all_posts(collection).into_iter().filter(|post: &Post| post.author.id == id).collect::<Vec<Post>>()
}
fn get_posts_by_category(collection: &Response, _category: &str) -> Vec<Post> {
    // todo: this only works half the time
    // how do I index a stuct dynamically
    // collection[category] is not a thing

    collection.diy.posts.clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retrieves_all_posts() {
        let source: Response = serde_json::from_str(
            &fs::read_to_string("./src/response.json").expect("Uh oh! I can\'t open the file."),
        )
        .unwrap();
        assert_eq!(get_all_posts(&source).len(), 4);
    }
    #[test]
    fn retrieves_a_post() {
        let source: Response = serde_json::from_str(
            &fs::read_to_string("./src/response.json").expect("Uh oh! I can\'t open the file."),
        )
        .unwrap();
        assert_eq!(get_post_by_id(&source, 1).title, "First Post!");
    }
    #[test]
    fn retrieves_posts_by_author_id() {
        let source: Response = serde_json::from_str(
            &fs::read_to_string("./src/response.json").expect("Uh oh! I can\'t open the file."),
        )
        .unwrap();
        assert_eq!(get_posts_by_author_id(&source, 1).len(), 3);
    }
    #[test]
    fn retrieves_posts_by_category_key() {
        let source: Response = serde_json::from_str(
            &fs::read_to_string("./src/response.json").expect("Uh oh! I can\'t open the file."),
        )
        .unwrap();
        assert_eq!(get_posts_by_category(&source, "diy").len(), 42);
    }
}
