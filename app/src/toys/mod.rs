use leptos::*;
use leptos_router::A;
mod ev_charge_sim;
pub use ev_charge_sim::VehicleSim;

#[component]
pub fn ToyPage() -> impl IntoView {
    view! {
        <div class="flex flex-col">
            <h2 class="text-2xl">"Toys"</h2>
            <A href="ev-charger-sim" class="text-lg hover:text-neutral-400">"EV charger sim"</A>
        </div>
    }
}
