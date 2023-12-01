<div id="top"></div>

<p align="center">
    <a href="https://github.com/https://github.com/pscott31/square_api/stargazers">
        <img src="https://img.shields.io/github/stars/https://github.com/pscott31/square_api.svg?style=flat-square" alt="Stars" />
    </a>
    <a href="https://github.com/https://github.com/pscott31/square_api/actions">
        <img src="https://img.shields.io/github/workflow/status/https://github.com/pscott31/square_api/ci?style=flat-square" alt="Build Status" />
    </a>
    
<a href="https://crates.io/crates/square_api">
    <img src="https://img.shields.io/crates/d/square_api?style=flat-square" alt="Downloads" />
</a>
<a href="https://crates.io/crates/square_api">
    <img src="https://img.shields.io/crates/v/square_api?style=flat-square" alt="Crates.io" />
</a>

</p>

SquareApi client, generated from the OpenAPI spec.

# Usage

```rust
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let response = client
        .create_mobile_authorization_code()
        .location_id("your location id")
        .await
        .unwrap();
    println!("{:#?}", response);
}
```

This example loads configuration from environment variables, specifically:



# Installation

Add this to your Cargo.toml:

```toml
[dependencies]
square_api = "0.1"
```


# Documentation



* [Client Library Documentation](https://docs.rs/square_api)


You can see working examples of every API call in the `examples/` directory.

# Contributing

Contributions are welcome!

*Library created with [Libninja](https://www.libninja.com).*