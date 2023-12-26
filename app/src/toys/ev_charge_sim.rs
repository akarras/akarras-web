use const_soft_float::soft_f64::SoftF64;
use itertools::Itertools;
use leptos::*;
use log::info;
use std::{
    collections::VecDeque,
    fmt::Display,
    iter::Sum,
    ops::{Add, AddAssign, Div, Mul, Sub},
    time::Duration,
};

// class="collapse"

use crate::components::Select;

/// Percent full represents a percent number from 0% to 100%, and will strictly enforce that.
/// Represented as a u16 from 0-10000 internally
/// Useful for representing state of charge
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct PercentFull(u16);

impl std::fmt::Debug for PercentFull {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("PercentFull")
            .field(&self.to_string())
            .finish()
    }
}

impl PercentFull {
    const PRECISION: f64 = 100.0;
    const fn new(float: f64) -> Self {
        // 100.0 * 100.0 -> 10000
        let percent = SoftF64(float).mul(SoftF64(Self::PRECISION)).0 as u16;
        Self(percent)
    }

    /// gets this percent as a float from 100.0 -> 0.0
    const fn as_float(&self) -> f64 {
        SoftF64(self.0 as f64)
            .div(SoftF64(Self::PRECISION))
            .to_f64()
    }

    /// gets this percent as a float from 1.0 -> 0.0
    const fn as_partial_float(&self) -> f64 {
        SoftF64(self.0 as f64)
            .div(SoftF64(Self::PRECISION).mul(SoftF64(100.0)))
            .to_f64()
    }
}

impl Display for PercentFull {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.2}%", self.as_float())
    }
}

impl Mul<Energy> for PercentFull {
    type Output = Energy;

    fn mul(self, rhs: Energy) -> Self::Output {
        Energy {
            watt_hours: rhs.watt_hours * self.as_partial_float(),
        }
    }
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Default)]
struct Energy {
    watt_hours: f64,
}

impl AddAssign for Energy {
    fn add_assign(&mut self, rhs: Self) {
        self.watt_hours += rhs.watt_hours;
    }
}

impl Energy {
    const fn from_kwh(kilowatts_per_hour: f64) -> Self {
        Self {
            watt_hours: (SoftF64(kilowatts_per_hour).mul(SoftF64(1000.0))).to_f64(),
        }
    }
}

impl Display for Energy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // lets just assume this is usually kwh for now.
        let kwh = self.watt_hours / 1000.0;
        write!(f, "{kwh:.1} KwH")
    }
}

impl Div<Power> for Energy {
    type Output = Duration;

    fn div(self, rhs: Power) -> Self::Output {
        let hours = (rhs.watts as f64) / self.watt_hours;
        Duration::from_secs_f64(hours / 60.0 / 60.0)
    }
}

impl Sum for Energy {
    fn sum<I: Iterator<Item = Self>>(mut iter: I) -> Self {
        let mut watt_hours = 0.0;
        while let Some(next) = iter.next() {
            watt_hours += next.watt_hours;
        }
        Energy { watt_hours }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
struct Power {
    watts: i32,
}

impl std::fmt::Debug for Power {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Power")
            .field("kw", &self.to_string())
            .finish()
    }
}

impl Power {
    const fn from_kw(kilowatts: f64) -> Self {
        let watts = SoftF64(kilowatts).mul(SoftF64(1000.0)).to_f64() as i32;
        Self { watts }
    }

    fn as_kw(&self) -> f64 {
        self.watts as f64 / 1000.0
    }
}

impl AddAssign for Power {
    fn add_assign(&mut self, rhs: Self) {
        self.watts += rhs.watts;
    }
}

impl Div<i32> for Power {
    type Output = Power;

    fn div(self, rhs: i32) -> Self::Output {
        Self {
            watts: self.watts / rhs,
        }
    }
}

impl Div<u32> for Power {
    type Output = Power;

    fn div(self, rhs: u32) -> Self::Output {
        Self {
            watts: self.watts / (rhs as i32),
        }
    }
}

impl Div<f64> for Power {
    type Output = Power;

    fn div(mut self, rhs: f64) -> Self::Output {
        self.watts = ((self.watts as f64) / rhs) as i32;
        self
    }
}

impl Mul<f64> for Power {
    type Output = Power;

    fn mul(mut self, rhs: f64) -> Self::Output {
        self.watts = (self.watts as f64 * rhs) as i32;
        self
    }
}

impl Mul<i32> for Power {
    type Output = Power;

    fn mul(mut self, rhs: i32) -> Self::Output {
        self.watts = self.watts + rhs;
        self
    }
}

impl Mul<Duration> for Power {
    type Output = Energy;

    fn mul(self, rhs: Duration) -> Self::Output {
        let hours = rhs.as_secs_f64() / 60.0 / 60.0;
        let watt_hours = self.watts as f64 * hours;
        Energy { watt_hours }
    }
}

impl Sub for Power {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self.watts -= rhs.watts;
        self
    }
}

impl Display for Power {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let power_kw = self.as_kw();
        write!(f, "{power_kw:.1} kw")
    }
}

impl Add for Power {
    type Output = Power;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.watts += rhs.watts;
        self
    }
}

impl Sum<Power> for Power {
    fn sum<I: Iterator<Item = Power>>(mut iter: I) -> Self {
        let mut watts = 0;
        while let Some(next) = iter.next() {
            watts += next.watts;
        }
        Power { watts }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
struct CurvePoint {
    state_of_charge: PercentFull,
    charge_power: Power,
}

impl CurvePoint {
    const fn new(percent: f64, power_kwh: f64) -> Self {
        Self {
            state_of_charge: PercentFull::new(percent),
            charge_power: Power::from_kw(power_kwh),
        }
    }
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Default)]
struct ChargeCurve {
    /// data points must cover from 0% to 100%
    data_points: &'static [CurvePoint],
}

impl ChargeCurve {
    /// calculates the average charge charge power
    fn average_power(&self) -> Power {
        let total_power = self
            .data_points
            .windows(2)
            .map(|points| {
                let (point_1, point_2) = match points {
                    [point1, point2] => (point1, point2),
                    _ => unreachable!("should always have two points"),
                };
                let start_watts = point_1.charge_power.watts;
                let end_watts = point_2.charge_power.watts;
                let start_percent = point_1.state_of_charge.as_partial_float();
                let end_percent = point_2.state_of_charge.as_partial_float();
                let span_length = end_percent - start_percent;
                ((start_watts + end_watts) / 2) as f64 * span_length
            })
            .sum::<f64>();
        Power {
            watts: total_power as i32,
        }
    }

    /// linearly interpolates the power between two different charge for the given SOC
    fn power_at(&self, percent: PercentFull) -> Power {
        let internal_soc = percent.0;
        if let Some(exact) = self
            .data_points
            .iter()
            .find(|p| p.state_of_charge == percent)
        {
            return exact.charge_power;
        }
        if let Some((a, b)) =
            self.data_points.iter().tuple_windows().find(|(a, b)| {
                a.state_of_charge.0 < internal_soc && internal_soc < b.state_of_charge.0
            })
        {
            let span_length =
                b.state_of_charge.as_partial_float() - a.state_of_charge.as_partial_float();
            let length = percent.as_partial_float() - a.state_of_charge.as_partial_float();
            info!("{a:?} {b:?} {} {}", span_length, length);
            // y = mx + b (simple slope)
            ((b.charge_power - a.charge_power) / span_length * length) + a.charge_power
        } else {
            unreachable!("invalid percent provided {}", percent);
        }
    }

    /// creates a new subset of a charge curve
    fn percent_to_percent(&self, start_percent: PercentFull, end_percent: PercentFull) -> Self {
        // TODO implement
        self.clone()
    }
}

/// Contains the specification for a vehicle
#[derive(Clone, Copy, PartialEq, PartialOrd, Default)]
struct VehicleSpec {
    name: &'static str,
    battery_max: Energy,
    charge_curve: ChargeCurve,
    epa_miles: f64,
}

impl Eq for VehicleSpec {}

#[derive(Clone)]
struct Vehicle {
    spec: VehicleSpec,
    current_charge: Energy,

    unplug_at: Energy,
}

impl Vehicle {
    fn new(spec: VehicleSpec, state_of_charge: Energy, unplug_at: Energy) -> Vehicle {
        Vehicle {
            spec,
            current_charge: state_of_charge,
            unplug_at,
        }
    }

    fn soc(&self) -> PercentFull {
        if self.current_charge.watt_hours <= 1.0 {
            return PercentFull(0);
        }
        let soc = self.current_charge.watt_hours / self.spec.battery_max.watt_hours * 100.0;
        PercentFull::new(soc)
    }

    fn unplug_at_soc(&self) -> PercentFull {
        if self.unplug_at.watt_hours <= 1.0 {
            return PercentFull(0);
        }
        let soc = self.unplug_at.watt_hours / self.spec.battery_max.watt_hours * 100.0;
        PercentFull::new(soc)
    }

    /// Returns the next charge request- None if wants to unplug
    fn get_next_power_request(&mut self, charger_available: Power) -> Option<Power> {
        if self.current_charge >= self.unplug_at {
            return None;
        }
        let soc = self.soc();
        Some(
            self.spec
                .charge_curve
                .power_at(soc)
                .min(charger_available)
                .max(Power::from_kw(5.0)),
        )
    }

    // Charges the battery and returns the amount of energy added
    fn charge(&mut self, power: Power, dt: Duration) -> Energy {
        let added_energy = power * dt;
        info!("{} * {} = {}", power, dt.as_secs(), added_energy);
        self.current_charge += added_energy;
        assert!(power.as_kw().is_sign_positive());
        info!("current charge: {}", self.current_charge);
        added_energy
    }
}

static VEHICLES: &'static [VehicleSpec] = &[
    VehicleSpec {
        name: "KIA EV6 Long Range AWD",
        battery_max: Energy::from_kwh(77.4),
        charge_curve: ChargeCurve {
            data_points: &[
                // TODO: refine this curve
                CurvePoint::new(0.00, 20.0),
                CurvePoint::new(2.0, 220.0),
                CurvePoint::new(45.0, 238.0),
                CurvePoint::new(50.0, 198.0),
                CurvePoint::new(55.0, 198.0),
                CurvePoint::new(60.0, 100.0),
                CurvePoint::new(70.00, 198.0),
                CurvePoint::new(78.0, 168.0),
                CurvePoint::new(80.0, 0.0),
                CurvePoint::new(82.0, 125.0),
                CurvePoint::new(100.0, 20.0),
            ],
        },
        epa_miles: 270.0,
    },
    VehicleSpec {
        name: "Lucid Air Grand Touring",
        battery_max: Energy::from_kwh(112.0),
        charge_curve: ChargeCurve {
            data_points: &[
                CurvePoint::new(0.00, 200.0),
                CurvePoint::new(2.0, 280.0),
                CurvePoint::new(10.0, 300.0),
                CurvePoint::new(20.0, 290.0),
                CurvePoint::new(80.0, 100.0),
                CurvePoint::new(100.0, 10.0),
            ],
        },
        epa_miles: 510.0,
    },
    VehicleSpec {
        name: "Chevy Bolt 2022",
        battery_max: Energy::from_kwh(65.0),
        charge_curve: ChargeCurve {
            data_points: &[
                CurvePoint::new(0.0, 55.0),
                CurvePoint::new(50.0, 55.0),
                CurvePoint::new(70.0, 33.0),
                CurvePoint::new(93.0, 26.0),
                CurvePoint::new(100.0, 5.0),
            ],
        },
        epa_miles: 259.0,
    },
    VehicleSpec {
        name: "Tesla Model 3 LR AWD 2021",
        battery_max: Energy::from_kwh(82.0),
        charge_curve: ChargeCurve {
            data_points: &[
                CurvePoint::new(0.0, 80.0),
                CurvePoint::new(8.0, 225.0),
                CurvePoint::new(11.0, 250.0),
                CurvePoint::new(20.0, 250.0),
                CurvePoint::new(21.0, 200.0),
                CurvePoint::new(25.0, 150.0),
                CurvePoint::new(30.0, 140.0),
                CurvePoint::new(45.0, 145.0),
                CurvePoint::new(100.0, 5.0),
            ],
        },
        epa_miles: 358.0,
    },
];

#[derive(Clone, Copy, Debug)]
enum LoadSharingStrategy {
    None,
    /// Share power evenly throughout the given plugs
    /// for ex 300kw -> 2 plugs = 150 kw per plug.
    Paired {
        number_of_plugs: u32,
    },
    /// Same as Even, but with the option to load share
    /// for ex: 250kw -> 4 plugs = 125 max per plug, but 62.5kw if the adjacent plug is utilized
    Split {
        number_of_plugs: u32,
    },
    /// Load sharing where the power can be sent to any charger in the given power step
    /// still limited to max power per plug
    Granular {
        power_step: Power,
        number_of_plugs: u32,
        max_per_plug: Power,
    },
}

#[component]
fn VehiclePicker(
    #[prop(into)] current_vehicle: Signal<Option<&'static VehicleSpec>>,
    #[prop(into)] set_vehicle: SignalSetter<Option<&'static VehicleSpec>>,
) -> impl IntoView {
    let vehicles = create_rw_signal(
        VEHICLES
            .into_iter()
            .map(|vehicle_spec| vehicle_spec)
            .collect::<Vec<_>>(),
    );
    view! {
        <Select items=vehicles.into() as_label=move |v| v.name.to_string() choice=current_vehicle set_choice=set_vehicle let:vehicle>
            <div class="flex flex-row gap-2">
                <span>{vehicle.battery_max.to_string()}</span>
                <span>{vehicle.charge_curve.average_power().to_string()}" avg"</span>
                <span>{vehicle.charge_curve.percent_to_percent(PercentFull::new(10.0), PercentFull::new(80.0)).average_power().to_string()}" 10->80% avg"</span>
            </div>
        </Select>
    }
}

#[component]
fn VehicleChooser(vehicles: RwSignal<VecDeque<Vehicle>>) -> impl IntoView {
    let (vehicle_spec, set_vehicle_spec) = create_signal(None);
    let specs = create_memo(move |_| {
        vehicle_spec()
            .map(|spec: &VehicleSpec| *spec)
            .unwrap_or_default()
    });
    let (start_energy, set_start_energy) = create_signal(PercentFull::new(10.0));
    let (unplug_at, set_unplug_at) = create_signal(PercentFull::new(80.0));
    view! {
        <div class="flex flex-col">
                <h4 class="text-xl">"Vehicle:"</h4>
                <div class="flex flex-row gap-1">
                    <VehiclePicker current_vehicle=vehicle_spec set_vehicle=set_vehicle_spec />
                    <div class="flex flex-col" class:invisible=move || vehicle_spec.with(|spec| spec.is_none())>
                        <span>"battery capacity: "{move || specs().battery_max.to_string()}</span>
                        <span>"average charge speed: "{move || specs().charge_curve.average_power().to_string()}</span>
                        <span>"average 10->80% charge speed: "{move || specs().charge_curve.percent_to_percent(PercentFull::new(10.0), PercentFull::new(80.0)).average_power().to_string()}</span>
                    </div>
                    <div class:invisible=move || vehicle_spec.with(|spec| spec.is_none()) class="flex flex-col">
                        <label for="battery-soc" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">"Charge start battery%: "{move || start_energy().to_string()}" "{move || (start_energy() * specs().battery_max).to_string()}</label>
                        <input id="battery-soc" type="range" class="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer dark:bg-gray-700" prop:value=move || start_energy().as_float().to_string() on:input=move |e| {
                            if let Ok(value) = event_target_value(&e).parse() {
                                if unplug_at.get_untracked().as_float() < value {
                                    set_unplug_at(PercentFull::new((value + 1.0).min(100.0)));
                                }
                                set_start_energy(PercentFull::new(value));
                            }
                        }/>
                    </div>
                    <div class:collapse=move || vehicle_spec.with(|spec| spec.is_none()) class="flex flex-col">
                        <label for="battery-soc" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">"Unplug at Battery SOC%: "{move || unplug_at().to_string()}" "{move || (unplug_at() * specs().battery_max).to_string()}</label>
                        <input id="battery-soc" type="range" class="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer dark:bg-gray-700" prop:value=move || unplug_at().as_float().to_string() on:input=move |e| {
                            if let Ok(value) = event_target_value(&e).parse() {
                                if start_energy.get_untracked().as_float() > value {
                                    set_start_energy(PercentFull::new((value - 1.0).max(0.0)));
                                }
                                set_unplug_at(PercentFull::new(value));
                            }
                        }/>
                    </div>

                    <button class:collapse=move || vehicle_spec.with(|spec| spec.is_none()) class="bg-neutral-600 p-1 border border-neutral-500 hover:bg-neutral-700 rounded h-7"
                        on:click=move |_| {
                            if let Some(current) = vehicle_spec.get_untracked() {
                                vehicles.update(|v| v.push_back(Vehicle::new(*current, start_energy.get_untracked() * current.battery_max, unplug_at.get_untracked() * current.battery_max)));
                                set_vehicle_spec(None);
                            }
                        }>
                        "ADD +"
                    </button>
                </div>
            </div>
    }
}

#[component]
fn VehicleList(vehicles: RwSignal<VecDeque<Vehicle>>) -> impl IntoView {
    view! {
        <div class="flex flex-col" class:collapse=move || vehicles.with(|v| v.is_empty())>
            <h2 class="text-xl">"Vehicles:"</h2>
            <ul class="list-disc">
            <For each={move || vehicles().into_iter().enumerate()}
                key=|(i, v)| (*i, v.spec.name)
                let:vehicle>
                <li>{vehicle.1.spec.name}" " {vehicle.1.soc().to_string()}" -> "{vehicle.1.unplug_at_soc().to_string()} <button class="bg-red-600 rounded w-10 border border-neutral-500" on:click=move |_| vehicles.update(|v| { v.remove(vehicle.0);})>"X"</button></li>
            </For>
            </ul>
        </div>
    }
}

#[component]
fn ChargerBuilder(chargers: RwSignal<Vec<Charger>>) -> impl IntoView {
    let (grid_connection, set_grid_connection) = create_signal(Power::from_kw(600.0));
    let load_share = create_rw_signal(LoadSharingStrategy::None);
    let (number_of_plugs, set_number_of_plugs) = create_slice(
        load_share,
        move |strategy| match strategy {
            LoadSharingStrategy::None => None,
            LoadSharingStrategy::Paired { number_of_plugs } => Some(*number_of_plugs),
            LoadSharingStrategy::Split { number_of_plugs } => Some(*number_of_plugs),
            LoadSharingStrategy::Granular {
                number_of_plugs, ..
            } => Some(*number_of_plugs),
        },
        move |strategy, plugs| match strategy {
            LoadSharingStrategy::None => {}
            LoadSharingStrategy::Paired { number_of_plugs } => *number_of_plugs = plugs,
            LoadSharingStrategy::Split { number_of_plugs } => *number_of_plugs = plugs,
            LoadSharingStrategy::Granular {
                number_of_plugs, ..
            } => *number_of_plugs = plugs,
        },
    );
    let (power_step, set_power_step) = create_slice(
        load_share,
        move |strategy| match strategy {
            LoadSharingStrategy::Granular { power_step, .. } => Some(*power_step),
            _ => None,
        },
        move |strategy, step| match strategy {
            LoadSharingStrategy::Granular { power_step, .. } => *power_step = step,
            _ => {}
        },
    );
    let (max_per_plug, set_max_per_plug) = create_slice(
        load_share,
        move |strategy| match strategy {
            LoadSharingStrategy::Granular { max_per_plug, .. } => Some(*max_per_plug),
            _ => None,
        },
        move |strategy, per_plug| match strategy {
            LoadSharingStrategy::Granular { max_per_plug, .. } => *max_per_plug = per_plug,
            _ => {}
        },
    );
    let btn_active = "rounded-sm bg-neutral-800 p-1 border border-neutral-600";
    let btn_inactive = "rounded-sm bg-neutral-700 p-1 border border-neutral-600";
    view! {
        <div class="flex flex-col">
                <h4 class="text-xl">"Charger: "</h4>
                <div class="grid grid-cols-2">
                    <div>
                        "Grid Connection: "{move || grid_connection().to_string()}
                    </div>
                    <div>
                        <input class="dark:bg-neutral-800 w-36" value=grid_connection.get_untracked().as_kw() on:input=move |e| {
                            if let Ok(kwh) = event_target_value(&e).parse::<f64>() {
                                set_grid_connection(Power::from_kw(kwh));
                            }
                        } />
                    </div>
                    <div class="col-span-2 gap-1">
                        "Load sharing strategy:"
                        <button class=move || if matches!(load_share(), LoadSharingStrategy::None) { btn_active } else { btn_inactive  } on:click=move |_| load_share.set(LoadSharingStrategy::None)>"None"</button>
                        <button class=move || if matches!(load_share(), LoadSharingStrategy::Paired { .. }) { btn_active } else { btn_inactive  } on:click=move |_| load_share.set(LoadSharingStrategy::Paired {
                            number_of_plugs: 2
                        })>"Even"</button>
                        <button class=move || if matches!(load_share(), LoadSharingStrategy::Split { .. }) { btn_active } else { btn_inactive  } on:click=move |_| load_share.set(LoadSharingStrategy::Split {
                            number_of_plugs: 2
                        })>"Split"</button>
                        <button class=move || if matches!(load_share(), LoadSharingStrategy::Granular { .. }) { btn_active } else { btn_inactive  }  on:click=move |_| load_share.set(LoadSharingStrategy::Granular {
                            number_of_plugs: 8,
                            power_step: Power::from_kw(50.0),
                            max_per_plug: Power::from_kw(400.0),
                        })>"Granular"</button>
                        <div class="grid grid-cols-2 gap-1" class:collapse=move || number_of_plugs().is_none()>
                            <span>
                                "Number of plugs:"
                                {move || number_of_plugs().unwrap_or(1)}
                            </span>
                            <input class="dark:bg-neutral-800 w-36 shrink" prop:value=move || number_of_plugs().unwrap_or_default() on:input=move |e| {
                                if let Ok(value) = event_target_value(&e).parse() {
                                    set_number_of_plugs(value);
                                }
                            }/>
                            "Avg power per plug: " {move || (grid_connection() / (if number_of_plugs().unwrap_or(1) == 0 { 1 } else { number_of_plugs().unwrap_or(1) }) as i32).to_string()}
                        </div>
                        <div class="grid grid-cols-2 gap-1" class:collapse=move || !matches!(load_share(), LoadSharingStrategy::Granular { .. })>
                            <span>
                                "Power step:"
                                {move || power_step().unwrap_or(Power::from_kw(1.0)).to_string()}
                            </span>
                            <input class="dark:bg-neutral-800 w-36 shrink" prop:value=move || power_step().unwrap_or_default().as_kw() on:input=move |e| {
                                if let Ok(value) = event_target_value(&e).parse() {
                                    set_power_step(Power::from_kw(value));
                                }
                            }/>
                            <span>
                                "Max per plug:"
                                {move || max_per_plug().unwrap_or(Power::from_kw(1.0)).to_string()}
                            </span>
                            <input class="dark:bg-neutral-800 w-36 shrink" prop:value=move || max_per_plug().unwrap_or_default().as_kw() on:input=move |e| {
                                if let Ok(value) = event_target_value(&e).parse() {
                                    set_max_per_plug(Power::from_kw(value));
                                }
                            }/>
                        </div>
                    </div>
                    <button class="bg-neutral-600" on:click=move |_| {
                        let strategy = load_share.get_untracked();
                        chargers.update(|u| u.push(Charger::new(grid_connection.get_untracked(), strategy)));
                        load_share.set(LoadSharingStrategy::None);
                    }>"+ Add charger"</button>
                </div>
            </div>
    }
}

#[component]
fn ChargerList(chargers: RwSignal<Vec<Charger>>) -> impl IntoView {
    view! {
        <div class:collapse=move || chargers.with(|c| c.is_empty())>
            <h3 class="text-xl">"Chargers: "</h3>
            <For each=move || chargers.get().into_iter().enumerate()
            key=|(i, _c)| *i
            let:charger>
            <div>
                {charger.1.grid_connection.to_string()}" "
                {format!("{:?}", charger.1.strategy)}
            </div>
            </For>
        </div>
    }
}

#[derive(Clone)]
struct ChargingVehicle {
    /// the power allocated by the charger to this vehicle currently
    allocated_power: Power,
    vehicle: Vehicle,
}

impl ChargingVehicle {
    fn summary(&self) -> ChargeFrame {
        let soc = self.vehicle.soc();
        let allocated_power = self.allocated_power;
        let spec = self.vehicle.spec;
        ChargeFrame {
            soc,
            allocated_power,
            spec,
        }
    }
}

#[derive(Clone)]
struct Charger {
    grid_connection: Power,
    strategy: LoadSharingStrategy,
    currently_charging: Vec<ChargingVehicle>,
}

impl Charger {
    fn new(grid_connection: Power, strategy: LoadSharingStrategy) -> Self {
        Self {
            grid_connection,
            strategy,
            currently_charging: vec![],
        }
    }

    fn add_vehicle(&mut self, vehicle: Vehicle) {
        self.currently_charging.push(ChargingVehicle {
            allocated_power: Power::from_kw(0.0),
            vehicle,
        });
    }

    fn num_plugs(&self) -> u32 {
        match self.strategy {
            LoadSharingStrategy::None => 1,
            LoadSharingStrategy::Paired { number_of_plugs } => number_of_plugs,
            LoadSharingStrategy::Split { number_of_plugs } => number_of_plugs,
            LoadSharingStrategy::Granular {
                number_of_plugs, ..
            } => number_of_plugs,
        }
    }

    fn has_free_plug(&self) -> bool {
        self.num_plugs() > self.currently_charging.len() as u32
    }

    fn update_power_requests(&mut self) {
        match self.strategy {
            LoadSharingStrategy::None => self.currently_charging.retain_mut(|c| {
                if let Some(next) = c.vehicle.get_next_power_request(self.grid_connection) {
                    c.allocated_power = next;
                    true
                } else {
                    false
                }
            }),
            LoadSharingStrategy::Paired { number_of_plugs } => {
                // assume optimal distribution of vehicles
                let number_boosted_plugs =
                    (number_of_plugs - self.currently_charging.len() as u32) / 2;
                let power_per_plug = self.grid_connection / number_of_plugs;
                self.currently_charging.retain_mut(|c| {
                    let power = if number_boosted_plugs > 0 {
                        power_per_plug * 2
                    } else {
                        power_per_plug
                    };
                    if let Some(next) = c.vehicle.get_next_power_request(power) {
                        c.allocated_power = next;
                        true
                    } else {
                        false
                    }
                });
            }
            LoadSharingStrategy::Split { number_of_plugs } => {
                let power_per_plug = self.grid_connection / number_of_plugs;
                self.currently_charging.retain_mut(|c| {
                    if let Some(next) = c.vehicle.get_next_power_request(power_per_plug) {
                        c.allocated_power = next;
                        true
                    } else {
                        false
                    }
                });
            }
            LoadSharingStrategy::Granular {
                power_step,
                max_per_plug,
                ..
            } => {
                let mut allocated_power = self
                    .currently_charging
                    .iter()
                    .map(|c| power_step * (c.allocated_power.as_kw() / power_step.as_kw()).ceil())
                    .sum::<Power>();
                let total_power = self.grid_connection;
                self.currently_charging.retain_mut(|c| {
                    if let Some(power) = c.vehicle.get_next_power_request(
                        (c.allocated_power + (total_power - allocated_power)).min(max_per_plug),
                    ) {
                        let previous_rounded_power =
                            power_step * (c.allocated_power.as_kw() / power_step.as_kw()).ceil();
                        let current_rounded_power =
                            power_step * (power.as_kw() / power_step.as_kw()).ceil();
                        // 250 += 300 - 250 = 300
                        allocated_power += current_rounded_power - previous_rounded_power;
                        c.allocated_power = power;
                        true
                    } else {
                        // return our power to the pool
                        allocated_power +=
                            power_step * (c.allocated_power.as_kw() / power_step.as_kw()).ceil();
                        false
                    }
                });
            }
        }
    }

    fn charge_vehicles(&mut self, dt: Duration) -> Energy {
        self.currently_charging
            .iter_mut()
            .map(|vehicle| vehicle.vehicle.charge(vehicle.allocated_power, dt))
            .sum()
    }
}

struct Sim {
    /// all of the vehicles that are waiting to be charged
    vehicles: VecDeque<Vehicle>,
    /// all of the chargers in the simulation
    chargers: Vec<Charger>,
    /// length of time each step should simulate
    simulation_step_time: Duration,
    /// Total duration the simulation has simulated
    simulation_time: Duration,
}

struct ChargeFrame {
    soc: PercentFull,
    allocated_power: Power,
    spec: VehicleSpec,
}

struct SimFrame {
    energy_dispensed: Energy,
    vehicles_charging: Vec<ChargeFrame>,
    plugs_unused: u32,
    duration: Duration,
}

impl Sim {
    fn step(&mut self) -> SimFrame {
        // start charging any vehicles we can
        if !self.vehicles.is_empty() {
            for charger in self.chargers.iter_mut().filter(|c| c.has_free_plug()) {
                while charger.has_free_plug() && !self.vehicles.is_empty() {
                    charger.add_vehicle(self.vehicles.pop_front().unwrap());
                }
            }
        }
        // update power requests
        for charger in &mut self.chargers {
            charger.update_power_requests();
        }
        let energy_dispensed = self
            .chargers
            .iter_mut()
            .map(|c| c.charge_vehicles(self.simulation_step_time))
            .sum::<Energy>();
        self.simulation_time += self.simulation_step_time;
        SimFrame {
            energy_dispensed,
            vehicles_charging: self
                .chargers
                .iter()
                .flat_map(|c| c.currently_charging.iter().map(|c| c.summary()))
                .collect(),
            plugs_unused: self
                .chargers
                .iter()
                .map(|c| c.num_plugs() - c.currently_charging.len() as u32)
                .sum(),
            duration: self.simulation_time,
        }
    }

    fn is_valid(&self) -> bool {
        !self.chargers.is_empty()
    }

    fn is_done(&self) -> bool {
        self.vehicles.is_empty()
            && self
                .chargers
                .iter()
                .all(|c| c.currently_charging.is_empty())
    }
}

#[component]
fn Simulation(
    vehicles: RwSignal<VecDeque<Vehicle>>,
    chargers: RwSignal<Vec<Charger>>,
    sim_step: ReadSignal<Duration>,
) -> impl IntoView {
    {
        move || {
            let v = vehicles();
            let c = chargers();
            let simulation_step_time = sim_step();
            let mut sim = Sim {
                vehicles: v,
                chargers: c,
                simulation_step_time,
                simulation_time: Duration::default(),
            };
            let mut steps = vec![];
            if sim.is_valid() {
                while !sim.is_done() {
                    steps.push(sim.step());
                }
                let total_energy_dispensed =
                    steps.iter().map(|s| s.energy_dispensed).sum::<Energy>();
                let total_time_spent = steps.last().map(|s| s.duration).unwrap_or_default();
                view!{
                <div class="flex flex-col">
                <div>"energy dispensed: "{total_energy_dispensed.to_string()}</div><div>"minutes running: "{total_time_spent.as_secs()/60}</div><div>"----"</div>
                <div>{vehicles.with(|v| v.len())}" vehicles"</div><div>{chargers.with(|c| c.len())}" chargers"</div><div>{simulation_step_time.as_secs().to_string()}"s step time"</div><div></div>
                </div>
                <div class="grid grid-cols-4">
                    <div>"minutes"</div><div>"energy dispensed"</div><div>"vehicles charging"</div><div>"plugs unused"</div>
                    {steps.into_iter().map(|s| view!{
                        <div>{format!("{:.2}", s.duration.as_secs_f64() / 60.0)}</div>
                        <div>{s.energy_dispensed.to_string()}</div>
                        <div>{s.vehicles_charging.len()}" total"
                            <ul>
                                {s.vehicles_charging.into_iter().map(|summary| view!{ <li>{summary.allocated_power.to_string()}"@"{summary.soc.to_string()}", "{summary.spec.name.to_string()}</li>}).collect_view()}
                            </ul>
                        </div>
                        <div>{s.plugs_unused}</div>
                    }).collect_view()}
                </div>
            }.into_view()
            } else {
                view! { "Add chargers and vehicles to get started" }.into_view()
            }
        }
    }
}

#[component]
pub fn VehicleSim() -> impl IntoView {
    let vehicles = create_rw_signal(VecDeque::new());
    let chargers = create_rw_signal(vec![]);
    let (simulation_time, _) = create_signal(Duration::from_secs(15));
    view! {
        <div class="flex flex-col gap-2">
            <div class="flex flex-col gap-1">
                <h2 class="text-2xl">"EV Charging Simulator"</h2>
                <span>"Simulate real charging time for electric vehicles in the real world with a variety of chargers."</span>
            </div>
            <div class="flex flex-col gap-1">
                <VehicleChooser vehicles />
                <VehicleList vehicles />
            </div>
            <div class="flex flex-col gap-1">
                <ChargerBuilder chargers />
                <ChargerList chargers />
            </div>
            <div class="flex flex-col gap-1">
                <Simulation vehicles chargers sim_step=simulation_time />
            </div>
        </div>
    }
}
