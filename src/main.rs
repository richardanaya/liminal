use axum::{extract::Query, response::Html, routing::get, Extension, Router};

use serde::Deserialize;

use clap::{command, Parser};

#[derive(Deserialize)]
struct LiminalWeb {
    url: Option<String>,
}

#[derive(Parser, Debug, Clone, PartialEq)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short)]
    model: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .layer(Extension(args));

    println!("Server running on http://localhost:1111");
    // run our app with hyper, listening globally on port 1111
    let listener = tokio::net::TcpListener::bind("0.0.0.0:1111").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root(pagination: Query<LiminalWeb>, Extension(args): Extension<Args>) -> Html<String> {
    // get query param "url" from request
    if let Some(url) = &pagination.url {
        let client = reqwest::Client::new();
        let c = format!(
            r#"{{
                "model": {},
                "prompt": "Generate HTML for a website with url {}. Make sure there's some user content relevant to the site. Always have at least 5 links to sub pages on the main pages topic (not including the footer). No css stylesheets and only minimal color (like background). No images. No javascript. All links/form action urls on page should be prefixed with http://localhost:1111/?url=<full url goes here of link> . Links don't use targets. Just give me the HTML and make no commentary about the result and use no markdown/wiki annotation like (ie ```html).",
                "stream": false
            }}"#,
            args.model, &url
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
        // remove ```html
        let response = response.replace("```html", "");
        // remove ```
        let response = response.replace("```", "");
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
