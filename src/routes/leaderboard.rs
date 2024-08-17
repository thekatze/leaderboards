use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    Form, Json,
};
use maud::html;
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, types::Uuid};

use crate::{
    leaderboard::{HighScore, HighScores, Leaderboard},
    routes::{header, subheader},
    AppContext,
};

use super::layout;

async fn get_leaderboard_and_high_scores(
    leaderboard_id: &Uuid,
    context: &AppContext,
    limit: i64,
) -> Result<(Leaderboard, Vec<HighScore>), StatusCode> {
    let leaderboard_fut = query_as::<_, Leaderboard>("SELECT * FROM leaderboards WHERE id = ?1")
        .bind(leaderboard_id)
        .fetch_one(&context.db);

    let highscore_fut = query_as::<_, HighScore>(
        "SELECT * FROM highscores WHERE leaderboard_id = ?1 ORDER BY score DESC LIMIT ?2",
    )
    .bind(leaderboard_id)
    .bind(limit)
    .fetch_all(&context.db);

    let (leaderboard, high_scores) =
        tokio::try_join!(leaderboard_fut, highscore_fut).map_err(|e| match e {
            sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
            e => {
                tracing::error!("{:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            }
        })?;

    Ok((leaderboard, high_scores))
}

#[derive(Deserialize)]
pub(crate) struct Limits {
    limit: Option<i64>,
}

pub(crate) async fn render(
    Path(leaderboard_id): Path<String>,
    State(context): State<AppContext>,
    Query(limits): Query<Limits>,
) -> Result<impl IntoResponse, StatusCode> {
    let leaderboard_id = Uuid::try_parse(&leaderboard_id).map_err(|_| StatusCode::BAD_REQUEST)?;

    let (leaderboard, highscores) =
        get_leaderboard_and_high_scores(&leaderboard_id, &context, limits.limit.unwrap_or(20))
            .await?;

    let high_scores = HighScores {
        high_scores: highscores.into_boxed_slice(),
    };

    Ok(layout(html!(
        (header(leaderboard.name))
        div .flex .flex-col ."md:flex-row" .gap-8 {
            section .flex .flex-col .gap-4 .flex-1 .items-center {
                (subheader("High Scores"))
                (high_scores)
            }
        }

    )))
}

#[derive(Serialize)]
pub(crate) struct GetJsonResponseHighscore {
    username: String,
    score: i64,
}

#[derive(Serialize)]
pub(crate) struct GetJsonResponse {
    name: String,
    scores: Vec<GetJsonResponseHighscore>,
}

pub(crate) async fn get_json(
    Path(leaderboard_id): Path<String>,
    State(context): State<AppContext>,
    Query(limits): Query<Limits>,
) -> Result<impl IntoResponse, StatusCode> {
    let leaderboard_id = Uuid::try_parse(&leaderboard_id).map_err(|_| StatusCode::BAD_REQUEST)?;

    let (leaderboard, highscores) =
        get_leaderboard_and_high_scores(&leaderboard_id, &context, limits.limit.unwrap_or(20))
            .await?;

    Ok(Json(GetJsonResponse {
        name: leaderboard.name,
        scores: highscores
            .into_iter()
            .map(|highscore| GetJsonResponseHighscore {
                username: highscore.username,
                score: highscore.score,
            })
            .collect(),
    }))
}

#[derive(Debug, Deserialize)]
pub(crate) struct NewLeaderboardForm {
    name: String,
}
pub(crate) async fn handle_new(
    State(context): State<AppContext>,
    Form(form): Form<NewLeaderboardForm>,
) -> Result<impl IntoResponse, StatusCode> {
    let id = Uuid::new_v4();
    query("INSERT INTO leaderboards(id, name) VALUES (?1, ?2)")
        .bind(id)
        .bind(form.name)
        .execute(&context.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Redirect::to(&id.to_string()))
}
