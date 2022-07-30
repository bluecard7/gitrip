use std::collections::HashMap;
use warp;
use git2::Repository;

#[tokio::main]
async fn main() {
    let mut connection: GitConnection;
    let connect_filter = warp::path("connect")
        .and(warp::path::param())
        .map(|repo| {
            connection = connect(repo);
        });
    warp::post().and(connect_filter);
}

struct GitConnection {
    path: String;
}

impl GitConnection {
    fn new(repo: String) -> Self {
        GitConnection{path: repo}
    }
}

fn connect(url: String) -> Result<GitConnection, Error> {
    match Repository::open(url) {
        Ok(repo) => repo,
        Err(open_err) => {
            match Repository::close(url, "/dev") {
                Ok(repo) => repo,
                Err(clone_err) => /* something */,
            }
        }
    };
}
