// This file sets up a simple web server using the warp framework to respond with "Bonjour Léon!" on localhost.

use warp::Filter;

#[tokio::main]
async fn main() {
    // Define a route that serves a simple HTML page
    let route = warp::path::end()
        .map(|| warp::reply::html("<html><body>Bonjour Léon!</body></html>"));

    // Start the server on localhost:3030
    warp::serve(route)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
