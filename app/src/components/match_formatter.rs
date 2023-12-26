use leptos::*;
use sublime_fuzzy::Match;

/// Leptos version of sublime_fuzzy::format_simple
#[component]
pub fn MatchFormatter(m: Match, target: String) -> impl IntoView {
    let mut pieces = Vec::new();

    let mut last_end = 0;

    for c in m.continuous_matches() {
        // Piece between last match and this match
        pieces.push(
            view! {
            {target
                .chars()
                .skip(last_end)
                .take(c.start() - last_end)
                .collect::<String>()}}
            .into_view(),
        );

        // This match
        pieces.push(
            view! {<b>{target.chars().skip(c.start()).take(c.len()).collect::<String>()}</b>}
                .into_view(),
        );

        last_end = c.start() + c.len();
    }

    // Leftover chars
    if last_end != target.len() {
        pieces.push(
            target
                .chars()
                .skip(last_end)
                .collect::<String>()
                .into_view(),
        );
    }

    pieces
}
