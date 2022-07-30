use tokio;
use warp::{self, Filter};
use git2::Repository;

#[tokio::main]
async fn main() {
    let connect_filter = warp::post()
        .and(warp::path("api"))
        .and(warp::path("v0"))
        .and(warp::path::param())
        .map(|repo: String| {
            repo
        });
    println!("{}", std::env::current_dir().unwrap().to_str().unwrap());
    warp::serve(connect_filter)
        .run(([127, 0, 0, 1], 3000))
        .await;
}


fn connect(url: &str) -> Result<git2::Repository, (git2::Error, git2::Error)> {
    match Repository::open(url) {
        Ok(repo) => Ok(repo),
        Err(open_err) => {
            match Repository::clone(url, "/dev") {
                Ok(repo) => Ok(repo),
                Err(clone_err) => Err((open_err, clone_err)),
            }
        }
    }
}
