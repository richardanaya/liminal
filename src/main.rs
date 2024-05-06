use axum::{extract::Query, response::Html, routing::get, Router};

use serde::Deserialize;

#[derive(Deserialize)]
struct LiminalWeb {
    url: Option<String>,
}

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root));

    println!("Server running on http://localhost:1111");
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:1111").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root(pagination: Query<LiminalWeb>) -> Html<String> {
    // get query param "url" from request
    if let Some(url) = &pagination.url {
        let client = reqwest::Client::new();
        let c = format!(
            r#"{{
                "model": "phi3",
                "prompt": "HTML for a website with url {} that looks like a realistic website but surreally different. Fill with user content. Make no references to this being a fake page. In this alternate internet the world is happy and friendly. No css stylsheets and only minimal color (like background). No images. No javascript. All links/form action urls on page should be prefixed with http://localhost:3000/?url=<full url goes here of link> . Links don't use targets. Just give me the HTML and make no commentary about the result and use no markdown/wiki annotation like (ie ```html).",
                "stream": false
            }}"#,
            &url
        );
        let res = client
            .post("http://localhost:11434/api/generate")
            .body(c)
            .send()
            .await
            .unwrap();
        let text: String = res.text().await.unwrap();
        // parse JSON as generic object
        let result = serde_json::from_str::<serde_json::Value>(&text).unwrap();
        // get 'response' property from JSON
        let response = result["response"].as_str().unwrap();
        return response.to_string().into();
    }

    // let url = req.query("url").unwrap_or("https://www.google.com");
    r#"
    <html>
        <head>
            <title>Liminal</title>
        </head>
        <body>
            <h1>You are about to enter Liminal. An internet that never was, be careful you don't get lost.</h1>
            <form>
                <input type="text" name="url" placeholder="URL">
                <button type="submit">Go</button>
            </form>
        </body>
    </html>
    "#.to_string().into()
}
