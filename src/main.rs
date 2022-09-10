use std::convert::Infallible;
use std::net::SocketAddr;
use std::time::SystemTime;
use hyper::{Body, Request, Response, Server, StatusCode};
use hyper::service::{make_service_fn, service_fn};

async fn get_response(filepath: Option<&str>, response: &mut Response<Body>, uri: &str){
    match filepath{
        Some(filepath) => {
            match tokio::fs::read(filepath).await{
                Err(e) =>{
                    *response.body_mut() = Body::from(format!("Failed to read file with error: {}", e));
                    *response.status_mut() = StatusCode::NOT_FOUND;
                    println!("File \"{}\" not found!", filepath);
                },
                Ok(result) => {
                    *response.body_mut() = Body::from(result);
                    println!("File \"{}\" found", filepath);
                }
            };
        },
        None =>{
            *response.body_mut() = Body::from("Invalid request!");
            *response.status_mut() = StatusCode::BAD_REQUEST;
            println!("Invalid uri: \"{}\"", uri);
        }
    }
}

async fn michaeljoy(req: Request<Body>) -> Result<Response<Body>, hyper::Error>{
    let start = SystemTime::now();

    let uri_path = req.uri().path();
    println!("Got request: {}", uri_path);

    // start with a minimal response
    let mut response = Response::new(Body::empty());
    let files = tokio::fs::read_to_string("files.json").await.expect("Failed to read files.json");
    let files = json::parse(&files).expect("Failed to parse files.json");

    let filepath = files[uri_path].as_str();

    get_response(filepath, &mut response, uri_path).await;
    
    match start.elapsed(){
        Ok(elapsed) => println!("Finished request in {} seconds!\n", elapsed.as_secs_f64()),
        Err(e) => eprintln!("Failed to measure performance: {}\n", e)
    }
    Ok(response)
}

async fn shutdown_signal(){
    // wait for CTRL + C signal
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

#[tokio::main]
async fn main() {
    // we will bind to 192.168.178.12:8000, this will be the address it listens to
    let address = SocketAddr::from(([0, 0, 0, 0], 8000));
    
    // A service is needed for every connection,
    // so this creates one from our `hello_world` function
    let make_svc = make_service_fn(|_conn| async{
        Ok::<_, Infallible>(service_fn(michaeljoy))
    });

    // bind the server to the socket address, serving make_svc
    let server = Server::bind(&address).serve(make_svc);

    let graceful = server.with_graceful_shutdown(shutdown_signal());

    println!("Listening on port: 8000");
    // run the server for forever
    if let Err(e) = graceful.await{
        eprintln!("Server error: {}", e);
    }
}
