use clap::Parser;
use rust_embed::RustEmbed;
use tiny_http::{Header, Response, Server};

#[derive(Parser)]
struct Args {
    #[arg(long, default_value = "127.0.0.1")]
    host: String,
    #[arg(long, default_value_t = 9254)]
    port: u16,
}

#[derive(RustEmbed)]
#[folder = "dist"]
struct Assets;

fn main() {
    let args = Args::parse();
    let addr = format!("{}:{}", args.host, args.port);
    let server = match Server::http(&addr) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to bind to {addr}: {e}");
            eprintln!("Is another process already using port {}?", args.port);
            std::process::exit(1);
        }
    };

    let display_host = if args.host == "0.0.0.0" { "localhost" } else { args.host.as_str() };
    let url = format!("http://{}:{}/", display_host, args.port);
    println!("furiosa-opt-microscope is serving on {url}");
    match webbrowser::open(&url) {
        Ok(_) => println!("Opened {url} in your default browser."),
        Err(_) => println!("Could not open a browser automatically. Visit {url} manually."),
    }

    for request in server.incoming_requests() {
        let path = request.url().split('?').next().unwrap_or("/");
        let lookup = path.trim_start_matches('/');
        let lookup = if lookup.is_empty() { "index.html" } else { lookup };

        let (asset, served_path) = match Assets::get(lookup) {
            Some(a) => (a, lookup.to_string()),
            None => match Assets::get("index.html") {
                Some(a) => (a, "index.html".to_string()),
                None => {
                    let _ = request.respond(Response::from_string("404 Not Found").with_status_code(404));
                    continue;
                }
            },
        };

        let mime = mime_guess::from_path(&served_path)
            .first_or_octet_stream()
            .essence_str()
            .to_string();
        let header = Header::from_bytes(&b"Content-Type"[..], mime.as_bytes())
            .expect("content-type header should be valid");
        let response = Response::from_data(asset.data.into_owned()).with_header(header);
        let _ = request.respond(response);
    }
}