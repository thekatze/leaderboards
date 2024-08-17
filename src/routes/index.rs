use maud::{html, Markup};

use crate::routes::{header, submit_button};

use super::layout;

pub(crate) async fn render() -> Markup {
    layout(html!(
        (header("leaderboards"))
        p { "Easily create a leaderboard for game jam games." }
        form .flex .gap-4 action="/new" method="POST" {
            input .px-4 .py-2 .bg-indigo-100 .text-indigo-900 .rounded type="text" name="name" placeholder="Leaderboard name" required;
            (submit_button("Create"))
        }
    ))
}
