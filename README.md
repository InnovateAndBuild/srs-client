SRS Client
==========

[![srs-client](https://img.shields.io/badge/v0.1.0-blue) v0.1.0](https://github.com/InnovateAndBuild/srs-client/tree/v0.1.0) ([changelog](https://github.com/InnovateAndBuild/srs-client/blob/master/CHANGELOG.md))

The [SRS (Simple RTMP Server)][1] [Rust] Client or [srs-client][2] is a [Rust] package that provides bindings for the main functionalities of the SRS server. It supports two modes of operation:

1. As an HTTP client to interact with the SRS HTTP API
2. For handling SRS HTTP callbacks

## HTTP Client Mode

In this mode, srs-client uses HTTP to communicate with the server based on the [SRS HTTP API][3] specification.

### Usage

To use srs-client as an HTTP client:

```rust
use srs_client::{SrsClient, SrsClientError, SrsClientResp};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let srs_http_api_url = env::var("SRS_HTTP_API_URL").expect("SRS_HTTP_API_URL not set");
    let client = SrsClient::build(&srs_http_api_url)?;
    
    // Get SRS version
    let result: Result<SrsClientResp, SrsClientError> = client.get_version().await;
    println!("SRS version: {:?}", result);

    // Get streams
    let result: Result<SrsClientResp, SrsClientError> = client.get_streams().await;
    println!("Streams: {:?}", result);

    Ok(())
}
```

## HTTP Callback Mode

In this mode, srs-client provides structs to handle callbacks sent by SRS.

### Usage

To handle SRS callbacks:

```rust
use actix_web::{post, web};
use srs_client::{SrsCallbackEvent, SrsCallbackReq};

#[post("srs_callback")]
pub async fn on_callback(req: web::Json<SrsCallbackReq>) -> Result<&'static str, String> {
    match req.action {
        SrsCallbackEvent::OnConnect => {
            // Handle connection event
            dbg!(&req);
            Ok(())
        }
        _ => Ok(()),
    }
    .map(|()| "0")
}
```

## Features

- HTTP Client Mode:
  - Retrieve server information (version, vhosts, streams, clients)
  - Monitor system stats (rusages, self_proc_stats, system_proc_stats, meminfos)
  - Manage clients (kickoff)

- HTTP Callback Mode:
  - Handle various SRS events (OnConnect, OnPublish, etc.)

Full API Reference is available [here][4].

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
srs-client = "0.1.0"
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the [MIT License](LICENSE).

[Rust]: https://www.rust-lang.org
[1]: https://github.com/ossrs/srs
[2]: https://crates.io/crates/srs-client
[3]: https://ossrs.io/lts/en-us/docs/v5/doc/http-api
[4]: https://docs.rs/srs-client/
