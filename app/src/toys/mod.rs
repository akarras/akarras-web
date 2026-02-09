use leptos::prelude::*;
use leptos_router::components::A;
mod ev_charge_sim;
pub use ev_charge_sim::VehicleSim;

#[component]
pub fn ToyPage() -> impl IntoView {
    view! {
        <div class="flex flex-col py-8">
            <h2 class="text-3xl font-bold mb-4 gradient-text">"Toys"</h2>
            <A href="ev-charger-sim" attr:class="text-lg text-amber-600 dark:text-teal-400 hover:underline font-medium">"EV charger sim"</A>
        </div>
    }
}
