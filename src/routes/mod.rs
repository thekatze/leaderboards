use maud::{html, Markup, PreEscaped};

pub mod leaderboard;
pub mod highscores;
pub mod index;

fn header(content: impl maud::Render) -> Markup {
    html!(h1 .text-4xl .mb-8 .font-bold {(content)})
}

fn subheader(content: impl maud::Render) -> Markup {
    html!(h1 .text-2xl .text-indigo-100 .font-bold {(content)})
}

fn submit_button(content: impl maud::Render) -> Markup {
    html!(button .bg-indigo-100 .text-indigo-900 .font-black .px-4 .py-2 .rounded ."hover:bg-indigo-200" ."hover:text-indigo-950" type="submit" { (content) })
}

fn layout(page: impl maud::Render) -> Markup {
    html!(
        (maud::DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                title { "leaderboards" }
                style {
                    (PreEscaped(include_str!("styles_generated.css")))
                }
            }
            body .bg-indigo-950 .text-indigo-50 .font-medium {
                main .flex .flex-col .items-center .pt-16 .pb-4 .gap-8 .px-8 {
                    (page)
                }
            }
        }
    )
}
