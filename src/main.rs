use warp::Filter;
use serde_json::json;
use chrono::Utc;

#[tokio::main]
async fn main() {
    // Route GET /api/date
    let data_route = warp::path!("api" / "date")
        .map(move || {
            let current_date = Utc::now().to_rfc3339();
            warp::reply::json(&json!({"date": current_date}))
        });

    // Start the server on 127.0.0.1:8000
    warp::serve(data_route)
        .run(([0, 0, 0, 0], 8000))
        .await;
}
