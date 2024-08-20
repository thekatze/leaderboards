use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    Form,
};
use serde::Deserialize;
use sqlx::{query, query_as, types::Uuid};

use crate::{leaderboard::Leaderboard, AppContext};

use rustrict::CensorStr;

#[derive(Debug, Deserialize)]
pub(crate) struct AddHighscoreForm {
    username: String,
    score: i64,
}

pub(crate) async fn handle_add(
    Path(leaderboard_id): Path<String>,
    State(context): State<AppContext>,
    Form(form): Form<AddHighscoreForm>,
) -> Result<impl IntoResponse, StatusCode> {
    if form.username.len() > 16 {
        return Err(StatusCode::BAD_REQUEST);
    }

    let leaderboard_id = Uuid::try_parse(&leaderboard_id).map_err(|_| StatusCode::BAD_REQUEST)?;
    let leaderboard = query_as::<_, Leaderboard>("SELECT * FROM leaderboards WHERE id = ?1")
        .bind(leaderboard_id)
        .fetch_one(&context.db)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let sanitized_username = form.username.censor();

    let id = sqlx::types::uuid::Uuid::new_v4();
    query("INSERT INTO highscores(leaderboard_id, id, username, score) VALUES (?1, ?2, ?3, ?4)")
        .bind(leaderboard.id)
        .bind(id)
        .bind(sanitized_username)
        .bind(form.score)
        .execute(&context.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Redirect::to(&format!("/{}", leaderboard.id)))
}

#[derive(Deserialize)]
pub(crate) struct DeletePath {
    leaderboard_id: String,
    highscore_id: String,
}

pub(crate) async fn handle_delete(
    Path(path): Path<DeletePath>,
    State(context): State<AppContext>,
) -> Result<impl IntoResponse, StatusCode> {
    let leaderboard_id =
        Uuid::try_parse(&path.leaderboard_id).map_err(|_| StatusCode::BAD_REQUEST)?;
    let highscore_id = Uuid::try_parse(&path.highscore_id).map_err(|_| StatusCode::BAD_REQUEST)?;

    query("DELETE FROM highscores where leaderboard_id = ?1 AND id = ?2")
        .bind(leaderboard_id)
        .bind(highscore_id)
        .execute(&context.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}
