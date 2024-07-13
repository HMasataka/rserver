use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, patch},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let users = Users {
        users: vec![
            User {
                id: 1,
                name: "takashi".to_string(),
            },
            User {
                id: 2,
                name: "hitoshi".to_string(),
            },
            User {
                id: 3,
                name: "masashi".to_string(),
            },
        ],
    };
    let users_state = Arc::new(Mutex::new(users));

    let app = Router::new()
        .route("/", get(root))
        .route("/users", get(get_user).post(create_user))
        .route("/users/:user_id", patch(update_user).delete(delete_user))
        .with_state(users_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn get_user(State(users_state): State<Arc<Mutex<Users>>>) -> (StatusCode, Json<Users>) {
    let users_lock = users_state.lock().await;

    (StatusCode::OK, Json(users_lock.clone()))
}

async fn create_user(
    State(users_state): State<Arc<Mutex<Users>>>,
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<Users>) {
    let mut users_lock = users_state.lock().await;

    let user = User {
        id: (users_lock.users.len() + 1) as u32,
        name: payload.name.to_string(),
    };

    users_lock.users.push(user);

    (StatusCode::CREATED, Json(users_lock.clone()))
}

async fn update_user(
    State(users_state): State<Arc<Mutex<Users>>>,
    Path(user_id): Path<u32>,
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Result<Json<User>, String>) {
    let mut users_lock = users_state.lock().await;

    if let Some(user) = users_lock.users.iter_mut().find(|user| user.id == user_id) {
        user.name = payload.name.clone();

        return (StatusCode::OK, Ok(Json(user.clone())));
    }

    (StatusCode::BAD_REQUEST, Err("User not found".to_string()))
}

async fn delete_user(
    State(users_state): State<Arc<Mutex<Users>>>,
    Path(user_id): Path<u32>,
) -> Result<Json<Users>, String> {
    let mut users_lock = users_state.lock().await;

    let original_len = users_lock.users.len();

    users_lock.users.retain(|user| user.id != user_id);

    if users_lock.users.len() == original_len {
        return Err("User not found".to_string());
    }

    Ok(Json(users_lock.clone()))
}

#[derive(Deserialize)]
struct CreateUser {
    name: String,
}

#[derive(Clone, Deserialize, Serialize)]
struct User {
    id: u32,
    name: String,
}

#[derive(Clone, Deserialize, Serialize)]
struct Users {
    users: Vec<User>,
}
