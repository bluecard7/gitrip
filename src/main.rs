use std::fs;
use tokio;
use warp::{self, Filter, http::StatusCode};
use git2::Repository;

const REPOSITORY_HOME: &str = "./repositories";

#[tokio::main]
async fn main() {
    let current_dir = std::env::current_dir();
    fs::create_dir(current_dir.unwrap().join(REPOSITORY_HOME));

    let get_or_clone_repo = warp::post()
        .and(warp::path("api"))
        .and(warp::path("v0"))
        .and(warp::path::param())
        .map(|url: String| {
            get_repository(&url)
        });

    let list_repos = warp::get()
        .and(warp::path("api"))
        .and(warp::path("v0"))
        .and(warp::path("repositories"))
        .and_then(list_repositories);

    let routes = get_or_clone_repo.or(list_repos);
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3000))
        .await;
}

async fn list_repositories() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(
        match fs::read_dir(REPOSITORY_HOME) {
            Ok(dir_iter) => warp::reply::with_status(&dir_iter.collect(), StatusCode::OK),
            Err(_) => warp::reply::with_status("error reading repositories", StatusCode::INTERNAL_SERVER_ERROR),
        }
    )
}

async fn get_repository(url: &str) -> Result<impl warp::Reply, warp::Rejection> {
    match Repository::open(url) {
        Ok(repo) => Ok(warp::reply::with_status("found repo", StatusCode::OK)),
        Err(open_err) => {
            match Repository::clone(url, REPOSITORY_HOME) {
                Ok(repo) => Ok(warp::reply::with_status("cloned repo", StatusCode::OK)),
                Err(clone_err) => Ok(warp::reply::with_status("couldn't find repository + failed to clone", StatusCode::INTERNAL_SERVER_ERROR)),
            }
        }
    }
}
