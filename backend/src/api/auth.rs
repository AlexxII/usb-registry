use crate::AppState;
use axum::extract::State;
use axum::http::HeaderMap;
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use bcrypt::{DEFAULT_COST, hash, verify};
use jsonwebtoken::{DecodingKey, Validation, decode};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Deserialize)]
struct AuthRequest {
    password: String,
}

#[derive(Serialize)]
struct AuthResponse {
    token: String,
}

#[derive(Deserialize)]
struct ChangePasswordRequest {
    old_password: String,
    new_password: String,
}

const JWT_SECRET: &[u8] = b"my-ultra-secure-secret-key-12345";

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String, // Кого авторизовали (Subject)
    exp: u64,    // Время истечения токена в формате Unix Timestamp (обязательное поле для JWT)
}

pub fn route() -> Router<AppState> {
    Router::new()
        .route("/auth/admin", post(check_admin))
        .route("/auth/change-password", post(change_password))
        .route("/auth/verify", get(verify_token))
}

// Вспомогательная функция извлечения и валидации JWT из заголовков
fn verify_jwt(headers: &HeaderMap) -> Result<Claims, StatusCode> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if !auth_header.starts_with("Bearer ") {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = &auth_header[7..];
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::default(),
    )
    .map_err(|_| StatusCode::UNAUTHORIZED)?;

    Ok(token_data.claims)
}

// 1. ВХОД АДМИНИСТРАТОРА (Проверка пароля через БД)
async fn check_admin_ex(
    State(state): State<AppState>,
    Json(payload): Json<AuthRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
    // Получаем хэш пароля администратора из созданной таблицы
    let row: (String,) =
        sqlx::query_as("SELECT password_hash FROM admins WHERE role = 'admin' LIMIT 1")
            .fetch_one(&state.pool)
            .await
            .map_err(|e| {
                eprintln!("Ошибка БД при входе: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

    let db_password_hash = row.0;

    println!("Пришел пароль на бэк: [{}]", payload.password);
    println!("Длина пришедшего пароля: {}", payload.password.len());

    // Проверяем, соответствует ли введенный пароль хэшу из базы данных
    let is_valid = verify(&payload.password, &db_password_hash)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !is_valid {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Генерируем JWT-токен на 2 часа
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let claims = Claims {
        sub: "admin".to_string(),
        exp: current_time + (2 * 60 * 60),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(AuthResponse { token }))
}

async fn check_admin(
    State(state): State<AppState>,
    Json(payload): Json<AuthRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
    // 1. Получаем хэш пароля администратора из БД
    let row: (String,) =
        sqlx::query_as("SELECT password_hash FROM admins WHERE role = 'admin' LIMIT 1")
            .fetch_one(&state.pool)
            .await
            .map_err(|e| {
                eprintln!("Ошибка БД при входе: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

    let db_password_hash = row.0;

    // 2. Проверяем введенный пароль против хэша из БД
    let is_valid = verify(&payload.password, &db_password_hash)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !is_valid {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // 3. Генерируем JWT-токен на 2 часа
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let claims = Claims {
        sub: "admin".to_string(),
        exp: current_time + (2 * 60 * 60),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(AuthResponse { token }))
}

// 2. ПРОВЕРКА ТОКЕНА (Вызывается фронтендом Svelte при переходах)
async fn verify_token(headers: HeaderMap) -> Result<StatusCode, StatusCode> {
    verify_jwt(&headers)?;
    Ok(StatusCode::OK)
}

// 3. СМЕНА ПАРОЛЯ (С верификацией токена и обновлением хэша в БД)
async fn change_password(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<ChangePasswordRequest>,
) -> Result<StatusCode, StatusCode> {
    // Проверяем, что запрос делает авторизованный админ (валидируем токен)
    verify_jwt(&headers)?;

    // Достаем текущий хэш пароля из БД для сверки со старым паролем
    let row: (String,) =
        sqlx::query_as("SELECT password_hash FROM admins WHERE role = 'admin' LIMIT 1")
            .fetch_one(&state.pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let current_hash = row.0;

    // Сверяем введенный старый пароль с тем, что лежит в БД
    if !verify(&payload.old_password, &current_hash).unwrap_or(false) {
        return Err(StatusCode::BAD_REQUEST); // Старый пароль не совпадает
    }

    // Хэшируем новый пароль перед записью в БД
    let new_hash =
        hash(&payload.new_password, DEFAULT_COST).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Обновляем хэш пароля в базе данных
    sqlx::query("UPDATE admins SET password_hash = $1 WHERE role = 'admin'")
        .bind(new_hash)
        .execute(&state.pool)
        .await
        .map_err(|e| {
            eprintln!("Ошибка обновления пароля в БД: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(StatusCode::OK)
}

pub async fn ensure_default_admin(pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
    // Проверяем, есть ли уже админ в базе данных
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM admins WHERE role = 'admin'")
        .fetch_one(pool)
        .await?;

    // Если база данных пустая (первый запуск на проде)
    if count.0 == 0 {
        println!("База данных пуста. Генерация дефолтного администратора...");

        // Самое главное: Rust сам на проде создает хэш, совместимый с текущей ОС
        let default_hash =
            hash("12345", DEFAULT_COST).expect("Ошибка при хэшировании дефолтного пароля");

        // Записываем чистый хэш в БД
        sqlx::query("INSERT INTO admins (username, password_hash, role) VALUES ($1, $2, $3)")
            .bind("admin")
            .bind(default_hash)
            .bind("admin")
            .execute(pool)
            .await?;

        println!("=== ИНИЦИАЛИЗАЦИЯ УСПЕШНА ===");
        println!("Создан дефолтный пользователь: admin / 12345");
    }

    Ok(())
}
