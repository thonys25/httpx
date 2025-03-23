mod httpx;

use httpx::{
    HttpRequest,
    HttpResponse,
    StatusCode,
    Router
};

use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;
use async_std::task;

use serde::{
    Serialize,
    Deserialize
};

use serde_json as json;

#[derive(Debug, Serialize, Deserialize)]
struct Article {
    id: i32,
    title: String,
    text: String
}

#[async_std::main]
async fn main() {
    let mut router = Router::new();

    router.add_route("GET", "/", |req, res| {
        res.add_header("Content-Type", "text/html; charset=utf-8");
        res.write("Привет!");
    });

    router.add_route("GET", "/articles", |req, res| {
        let mut articles: Vec<Article> = vec![
            Article { id: 1, title: "Заголовок первой статьи".to_string(), text: "Текст описания первой статьи".to_string() },
            Article { id: 2, title: "Заголовок второй статьи".to_string(), text: "Описание для второй статьи".to_string() }
        ];

        let articles_json = match json::to_string(&articles) {
            Ok(json) => json,
            Err(err) => {
                res.add_header("Content-Type", "application/json; charset=utf-8");
                res.write(&format!("{}", json::json!({"status": 400})));
                return;
            },
        };

        res.add_header("Content-Type", "application/json; charset=utf-8");
        res.write(&articles_json);
    });

    router.add_route("GET", "/articles/:id", |req, res| {
        let id = req.get_param("id");
        res.add_header("Content-Type", "text/html; charset=utf-8");
        res.write(&format!("Вы запросили статью: {}", id));
    });

    router.add_route("GET", "/search", |req, res| {
        let search_query = req.get_query_param("q");
        res.add_header("Content-Type", "text/html; charset=utf-8");
        res.write(&format!("Поисковый запрос: {}", search_query));
    });

    router.add_route("POST", "/article/create", |req, res| {
        let form_data = req.get_body();

        let title = match form_data.get("title") {
            Some(value) => value.to_string(),
            None => String::new(),
        };

        let text = match form_data.get("text") {
            Some(value) => value.to_string(),
            None => String::new(),
        };

        println!("Title: {}", title);
        println!("Text: {}", text);
    });

    router.listen("127.0.0.1:8000").await;
}