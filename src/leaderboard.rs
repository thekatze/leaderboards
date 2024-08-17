use maud::{html, Markup};

#[derive(sqlx::FromRow)]
pub(crate) struct Leaderboard {
    pub id: sqlx::types::uuid::Uuid,
    pub(crate) name: String,
}

pub(crate) struct HighScores {
    pub(crate) high_scores: Box<[HighScore]>,
}

#[derive(sqlx::FromRow)]
pub(crate) struct HighScore {
    pub id: sqlx::types::uuid::Uuid,
    pub username: String,
    pub score: i64,
}

impl maud::Render for HighScores {
    fn render(&self) -> Markup {
        let has_events = !self.high_scores.is_empty();
        html!(
            ol .flex .flex-col .gap-4 .p-4 {
                @if has_events {
                    @for event in self.high_scores.iter() {
                        (event)
                    }
                } @else {
                    span .text-indigo-400 {"No Highscores yet"}
                }
            }
        )
    }
}

impl maud::Render for HighScore {
    fn render(&self) -> Markup {
        html!(
            li .flex .gap-4 id=(format!("highscore-{}", self.id)) {
                strong { (self.username.to_string()) } (self.score)
            }
        )
    }
}
