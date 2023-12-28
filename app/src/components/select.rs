use leptos::{
    html::{Div, Input},
    *,
};
use leptos_use::use_element_hover;
use sublime_fuzzy::{FuzzySearch, Match, Scoring};

use super::MatchFormatter;
pub(crate) fn fuzzy_search(query: &str, target: &str) -> Option<Match> {
    let scoring = Scoring::emphasize_distance();
    let search = FuzzySearch::new(query, target)
        .case_insensitive()
        .score_with(&scoring);
    search.best_match()
}

#[component]
pub fn Select<T, EF, N, L>(
    items: Signal<Vec<T>>,
    as_label: L,
    choice: Signal<Option<T>>,
    set_choice: SignalSetter<Option<T>>,
    children: EF,
) -> impl IntoView
where
    T: Clone + Eq + 'static,
    EF: Fn(T) -> N + 'static + Copy,
    N: IntoView + 'static,
    L: Fn(&T) -> String + 'static + Copy,
{
    let (current_input, set_current_input) = create_signal("".to_string());
    let (has_focus, set_focused) = create_signal(false);
    let dropdown = create_node_ref::<Div>();
    let input = create_node_ref::<Input>();
    let hovered = use_element_hover(dropdown);
    let labels = create_memo(move |_| {
        items.with(|i| {
            i.iter()
                .map(|i| as_label(i))
                .enumerate()
                .collect::<Vec<_>>()
        })
    });
    let search_results = create_memo(move |_| {
        current_input.with(|input| {
            let mut results = labels.with(|s| {
                s.iter()
                    .filter_map(|(i, label)| {
                        fuzzy_search(input, label).map(|m| (*i, label.clone(), m))
                    })
                    .collect::<Vec<_>>()
            });
            results.sort_by_key(|(_, _, l)| l.score());
            results
                .into_iter()
                .map(|(i, l, _)| (i, l))
                .collect::<Vec<_>>()
        })
    });
    let final_result = create_memo(move |_| {
        let search_results = search_results();
        if search_results.is_empty() {
            labels()
        } else {
            search_results
        }
    });
    // class="invisible"
    view! {
        <div class="relative">
            <input node_ref=input
                class:cursor=move || !has_focus()
                class="p-2 rounded-md bg-gray-200 dark:bg-gray-800 border-solid border border-gray-500 w-96 hover:bg-gray-300 dark:hover:bg-gray-700 hover:border-neutral-400 dark:hover:border-neutral-600"
                on:focus=move |_| set_focused(true)
                on:focusout=move |_| set_focused(false)
                on:input=move |e| { set_current_input(event_target_value(&e)); }
                prop:value=current_input />
            <div class="absolute top-2 left-2 select-none cursor" class:invisible=move || has_focus() || !current_input().is_empty() on:click=move |_| {
                if let Some(input) = input() {
                    let _ = input.focus();
                }
            }>
                {move || choice().map(|c| {
                    as_label(&c).into_view()
                })}
            </div>
            <div node_ref=dropdown class:invisible=move || !has_focus() && !hovered()
                class="focus-within:visible absolute w-96 h-96 overflow-y-auto top-10 bg-gray-100 dark:bg-gray-950 z-20">
                <For each=final_result
                    key=move |(l, _)| *l
                    let:data
                >
                    <button class="flex flex-col w-full" on:click=move |_| {
                        if let Some(item) = items.with(|i| i.get(data.0).cloned()) {
                            set_choice(Some(item));
                            set_focused(false);
                            set_current_input("".to_string());
                        }
                    }>
                        <div class="hover:bg-gray-300 dark:hover:bg-gray-700 hover:border-solid hover:border-neutral-200 dark:hover:border-neutral-600 rounded-sm p-2" class:bg-gray-500=move || {
                            choice.with(|choice| choice.as_ref().and_then(|choice| items.with(|i| i.get(data.0).map(|item| item == choice)))).unwrap_or_default()
                        }>{items.with(|i| i.get(data.0).cloned()).map(|c| {move || {
                            let view = if let Some(m) = fuzzy_search(&current_input(), &data.1){
                                let target = data.1.clone();
                                view!{ <div class="flex flex-row"><span><MatchFormatter m=m target=target /></span></div> }
                            } else {
                                view!{ <div class="flex flex-row">{&data.1}</div>}
                            };
                            view!{
                                {view}
                                {children(c.clone())}
                            }
                        }.into_view()})}</div>
                    </button>
                </For>
            </div>
        </div>
    }
}
