# httpx - The simplest web router for Rust.

## The main idea is to make a simple and intuitive router for the Rust language, which will be as simple as in Laravel or Go.

### Example of a simple application
```
mod httpx;

use httpx::{
	HttpRequest,
	HttpResponse,
	StatusCode,
	Router
};

use async_std::net::{TcpListener, TcpStream};
use async_std::prelude;
use async_std::task;

#[async_std::main]
async fn main() {
	let mut router = Router::new();

	router.add_route("GET", "/", |req, res| {
		res.write("Welcome to httpx!");
	});

	router.listen("127.0.0.1:8000").await;
}
```

Looks like very easy, do you agree?

You can see more examples in main.rs.

## Contribute