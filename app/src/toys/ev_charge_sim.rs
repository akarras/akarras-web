use base64::{engine::general_purpose, Engine};
use const_soft_float::soft_f64::SoftF64;
use flate2::{read::DeflateDecoder, write::DeflateEncoder, Compression};
use itertools::Itertools;
use leptos::{html, prelude::*};
use leptos_meta::{Script, Title};
use leptos_router::hooks::{use_location, use_navigate};
use leptos_router::NavigateOptions;
use leptos_use::{
    core::Size, use_element_size, use_element_size_with_options, use_media_query,
    use_preferred_dark, UseElementSizeOptions, UseElementSizeReturn,
};
use log::info;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{
    borrow::{Borrow, Cow},
    collections::VecDeque,
    fmt::Display,
    io::{Cursor, Read, Write},
    iter::{self, Sum},
    ops::{Add, AddAssign, Div, Mul, Sub},
    time::Duration,
};
use thiserror::Error;

// class="collapse"

use crate::components::Select;

/// Percent full represents a percent number from 0% to 100%, and will strictly enforce that.
/// Represented as a u16 from 0-10000 internally
/// Useful for representing state of charge
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct PercentFull(i16);

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
        let percent = SoftF64(float).mul(SoftF64(Self::PRECISION)).0 as i16;
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

impl Sub for PercentFull {
    type Output = PercentFull;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Default, Serialize, Deserialize)]
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

    fn as_kwh(&self) -> f64 {
        self.watt_hours / 1000.0
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
        let hours = self.watt_hours / (rhs.watts as f64);
        Duration::try_from_secs_f64(hours * 60.0 * 60.0).unwrap_or_default()
    }
}

impl Add for Energy {
    type Output = Energy;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            watt_hours: self.watt_hours + rhs.watt_hours,
        }
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize)]
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

#[derive(Clone, PartialEq, PartialOrd, Default)]
struct ChargeCurve {
    /// data points must cover from 0% to 100%
    data_points: Cow<'static, [CurvePoint]>,
}

impl ChargeCurve {
    /// calculates the average charge charge power
    fn average_power(&self) -> Power {
        if self.data_points.len() < 1 {
            return Power::from_kw(0.0);
        }
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
        let start = self.data_points.first().unwrap().state_of_charge;
        let end = self.data_points.last().unwrap().state_of_charge;
        let length = end - start;
        let length_correction = 1.0 / length.as_partial_float();
        let total_power = total_power * length_correction;
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
            // y = mx + b (simple slope)
            ((b.charge_power - a.charge_power) / span_length * length) + a.charge_power
        } else {
            unreachable!("invalid percent provided {}", percent);
        }
    }

    /// creates a new subset of a charge curve
    fn percent_to_percent(
        &self,
        start_percent: PercentFull,
        end_percent: PercentFull,
    ) -> Option<Self> {
        let ((_, _), (start_edge, _)) =
            self.data_points
                .iter()
                .enumerate()
                .tuple_windows()
                .find(|((_, a), (_, b))| {
                    a.state_of_charge.0 <= start_percent.0 && b.state_of_charge.0 > start_percent.0
                })?;
        let ((end_edge, _), (_, _)) =
            self.data_points
                .iter()
                .enumerate()
                .tuple_windows()
                .find(|((_, a), (_, b))| {
                    a.state_of_charge.0 < end_percent.0 && b.state_of_charge.0 >= end_percent.0
                })?;
        let curve_middle = &self.data_points[start_edge..=end_edge];
        let start_point = CurvePoint {
            state_of_charge: start_percent,
            charge_power: self.power_at(start_percent),
        };
        let end_point = CurvePoint {
            state_of_charge: end_percent,
            charge_power: self.power_at(end_percent),
        };
        let data = iter::once(start_point)
            .chain(curve_middle.iter().copied())
            .chain(iter::once(end_point))
            .collect::<Vec<_>>();
        Some(ChargeCurve {
            data_points: data.into(),
        })
    }
}

/// Contains the specification for a vehicle
#[derive(Clone, PartialEq, PartialOrd, Default)]
struct VehicleSpec {
    name: &'static str,
    battery_max: Energy,
    charge_curve: ChargeCurve,
    epa_miles: f64,
}

impl Eq for VehicleSpec {}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
struct SpecKey {
    name: Cow<'static, str>,
}

#[derive(Debug, Error)]
enum VehicleLookupError {
    #[error("Unable to find vehicle by name {0}")]
    NotFound(Cow<'static, str>),
}

impl TryFrom<&SpecKey> for &'static VehicleSpec {
    type Error = VehicleLookupError;

    fn try_from(value: &SpecKey) -> Result<Self, Self::Error> {
        VEHICLES
            .into_iter()
            .find(|v| v.name == value.name)
            .ok_or_else(|| VehicleLookupError::NotFound(value.name.clone()))
    }
}

impl From<&'static VehicleSpec> for SpecKey {
    fn from(value: &'static VehicleSpec) -> Self {
        Self {
            name: Cow::Borrowed(value.name),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
struct Vehicle {
    spec: SpecKey,
    current_charge: Energy,
    unplug_at: Energy,
}

impl Vehicle {
    fn new(spec: &'static VehicleSpec, state_of_charge: Energy, unplug_at: Energy) -> Vehicle {
        Vehicle {
            spec: spec.into(),
            current_charge: state_of_charge,
            unplug_at,
        }
    }

    fn spec_details(&self) -> &'static VehicleSpec {
        static DEFAULT: VehicleSpec = VehicleSpec {
            name: "",
            battery_max: Energy::from_kwh(0.0),
            charge_curve: ChargeCurve {
                data_points: Cow::Borrowed(&[]),
            },
            epa_miles: SoftF64(0.0).to_f64(),
        };
        self.spec.borrow().try_into().ok().unwrap_or(&DEFAULT)
    }

    fn soc(&self) -> PercentFull {
        if self.current_charge.watt_hours <= 1.0 {
            return PercentFull(0);
        }
        let soc =
            self.current_charge.watt_hours / self.spec_details().battery_max.watt_hours * 100.0;
        PercentFull::new(soc)
    }

    fn unplug_at_soc(&self) -> PercentFull {
        if self.unplug_at.watt_hours <= 1.0 {
            return PercentFull(0);
        }
        let soc = self.unplug_at.watt_hours / self.spec_details().battery_max.watt_hours * 100.0;
        PercentFull::new(soc)
    }

    /// Returns the next charge request- None if wants to unplug
    fn get_next_power_request(&mut self, charger_available: Power) -> Option<Power> {
        if self.current_charge >= self.unplug_at {
            return None;
        }
        let soc = self.soc();
        Some(
            self.spec_details()
                .charge_curve
                .power_at(soc)
                .min(charger_available)
                .max(Power::from_kw(5.0)),
        )
    }

    // Charges the battery and returns the amount of energy added
    fn charge(&mut self, power: Power, dt: Duration) -> Energy {
        let added_energy = power * dt;
        self.current_charge += added_energy;
        assert!(power.as_kw().is_sign_positive());
        added_energy
    }
}

static VEHICLES: &'static [VehicleSpec] = &[
    VehicleSpec {
        name: "KIA EV6 Long Range AWD",
        battery_max: Energy::from_kwh(77.4),
        charge_curve: ChargeCurve {
            data_points: Cow::Borrowed(&[
                // TODO: refine this curve
                CurvePoint::new(0.00, 20.0),
                CurvePoint::new(2.0, 220.0),
                CurvePoint::new(45.0, 238.0),
                CurvePoint::new(50.0, 198.0),
                CurvePoint::new(55.0, 198.0),
                CurvePoint::new(60.0, 100.0),
                CurvePoint::new(70.00, 198.0),
                CurvePoint::new(77.0, 75.0),
                CurvePoint::new(78.0, 168.0),
                CurvePoint::new(82.0, 10.0),
                CurvePoint::new(83.0, 125.0),
                CurvePoint::new(100.0, 20.0),
            ]),
        },
        epa_miles: 270.0,
    },
    VehicleSpec {
        name: "Lucid Air Grand Touring",
        battery_max: Energy::from_kwh(112.0),
        charge_curve: ChargeCurve {
            data_points: Cow::Borrowed(&[
                CurvePoint::new(0.00, 200.0),
                CurvePoint::new(2.0, 280.0),
                CurvePoint::new(10.0, 300.0),
                CurvePoint::new(20.0, 290.0),
                CurvePoint::new(80.0, 100.0),
                CurvePoint::new(100.0, 10.0),
            ]),
        },
        epa_miles: 510.0,
    },
    VehicleSpec {
        name: "Porsche Taycan 2022",
        battery_max: Energy::from_kwh(93.4),
        charge_curve: ChargeCurve {
            data_points: Cow::Borrowed(&[
                CurvePoint::new(0.00, 260.0),
                CurvePoint::new(21.0, 265.0),
                CurvePoint::new(22.0, 250.0),
                CurvePoint::new(28.0, 200.0),
                CurvePoint::new(80.0, 100.0),
                CurvePoint::new(100.0, 10.0),
            ]),
        },
        epa_miles: 510.0,
    },
    VehicleSpec {
        name: "Chevy Bolt 2022",
        battery_max: Energy::from_kwh(65.0),
        charge_curve: ChargeCurve {
            data_points: Cow::Borrowed(&[
                CurvePoint::new(0.0, 55.0),
                CurvePoint::new(50.0, 55.0),
                CurvePoint::new(70.0, 33.0),
                CurvePoint::new(93.0, 26.0),
                CurvePoint::new(100.0, 5.0),
            ]),
        },
        epa_miles: 259.0,
    },
    VehicleSpec {
        name: "Tesla Model 3 LR AWD 2021",
        battery_max: Energy::from_kwh(82.0),
        charge_curve: ChargeCurve {
            data_points: Cow::Borrowed(&[
                CurvePoint::new(0.0, 80.0),
                CurvePoint::new(8.0, 225.0),
                CurvePoint::new(11.0, 250.0),
                CurvePoint::new(20.0, 250.0),
                CurvePoint::new(24.0, 250.0),
                CurvePoint::new(26.0, 200.0),
                CurvePoint::new(34.0, 200.0),
                CurvePoint::new(36.0, 150.0),
                CurvePoint::new(66.0, 120.0),
                CurvePoint::new(69.0, 120.0),
                CurvePoint::new(80.0, 60.0),
                CurvePoint::new(100.0, 20.0),
            ]),
        },
        epa_miles: 358.0,
    },
    VehicleSpec {
        name: "Rivian R1S Standard Pack",
        battery_max: Energy::from_kwh(105.0),
        charge_curve: ChargeCurve {
            data_points: Cow::Borrowed(&[
                // There might be a revised charge curve for 2023 but I can't find a full sample
                CurvePoint::new(0.0, 100.0),
                CurvePoint::new(1.0, 190.0),
                CurvePoint::new(47.0, 230.0),
                CurvePoint::new(50.0, 173.0),
                CurvePoint::new(55.0, 147.0),
                CurvePoint::new(57.0, 175.0),
                CurvePoint::new(60.0, 145.0),
                CurvePoint::new(70.0, 75.0),
                CurvePoint::new(80.0, 75.0),
                CurvePoint::new(100.0, 15.0),
            ]),
        },
        epa_miles: 352.0,
    },
    VehicleSpec {
        name: "GMC Hummer EV Pickup",
        battery_max: Energy::from_kwh(212.0),
        charge_curve: ChargeCurve {
            data_points: Cow::Borrowed(&[
                // There might be a revised charge curve for 2023 but I can't find a full sample
                CurvePoint::new(0.0, 150.0),
                CurvePoint::new(1.0, 335.0),
                CurvePoint::new(2.0, 338.0),
                CurvePoint::new(34.0, 345.0),
                CurvePoint::new(36.0, 306.0),
                CurvePoint::new(40.0, 294.0),
                CurvePoint::new(50.0, 257.0),
                CurvePoint::new(62.0, 255.0),
                CurvePoint::new(70.0, 115.0),
                CurvePoint::new(80.0, 45.0),
                CurvePoint::new(83.0, 17.0),
                CurvePoint::new(90.0, 51.0),
                CurvePoint::new(100.0, 15.0),
            ]),
        },
        epa_miles: 352.0,
    },
];

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
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
fn VehicleDropdown(
    #[prop(into)] current_vehicle: Signal<Option<&'static VehicleSpec>>,
    #[prop(into)] set_vehicle: SignalSetter<Option<&'static VehicleSpec>>,
) -> impl IntoView {
    let vehicles = RwSignal::new(
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
                <span>{vehicle.charge_curve.percent_to_percent(PercentFull::new(10.0), PercentFull::new(80.0)).map(|curve| curve.average_power().to_string())}" 10->80% avg"</span>
            </div>
        </Select>
    }
}

#[component]
fn VehicleChooser(
    #[prop(into)] vehicles: Signal<VecDeque<Vehicle>>,
    set_vehicles: SignalSetter<VecDeque<Vehicle>>,
) -> impl IntoView {
    let (vehicle_spec, set_vehicle_spec) = signal::<Option<&'static VehicleSpec>>(None);
    let specs = Memo::new(move |_| {
        vehicle_spec()
            .map(|spec: &VehicleSpec| spec.clone())
            .unwrap_or_default()
    });
    let (start_energy, set_start_energy) = signal(PercentFull::new(10.0));
    let (unplug_at, set_unplug_at) = signal(PercentFull::new(80.0));
    let estimated_charge_time = move || {
        (unplug_at() - start_energy()) * specs().battery_max
            / specs()
                .charge_curve
                .percent_to_percent(start_energy(), unplug_at())
                .map(|curve| curve.average_power())
                .unwrap_or(Power::from_kw(0.0))
    };
    view! {
        <div class="flex flex-col">
                <h4 class="text-xl">"Add Vehicle:"</h4>
                <div class="flex flex-col xl:flex-row gap-1">
                    <VehicleDropdown current_vehicle=vehicle_spec set_vehicle=set_vehicle_spec />
                    <div class="flex flex-col" class:invisible=move || vehicle_spec.with(|spec| spec.is_none())>
                        <span>"battery capacity: "{move || specs().battery_max.to_string()}</span>
                        <span>"avg charge speed: "{move || specs().charge_curve.average_power().to_string()}</span>
                        <span>"avg "{move || start_energy().to_string()}"->"{move || unplug_at().to_string()}" charge speed:"<span>{move || specs().charge_curve.percent_to_percent(start_energy(), unplug_at()).map(|curve| curve.average_power().to_string())}</span></span>
                        <span>{move || format!("estimated charge time: {:.2} mins", (estimated_charge_time().as_secs_f64() / 60.0))}</span>
                    </div>
                    <div class:invisible=move || vehicle_spec.with(|spec| spec.is_none()) class="flex flex-col">
                        <label for="battery-soc" class="block mb-2 text-sm font-medium text-slate-700 dark:text-slate-200">"Charge start battery%: "{move || start_energy().to_string()}" "{move || (start_energy() * specs().battery_max).to_string()}</label>
                        <input id="battery-soc" type="range" class="w-full h-2 bg-amber-200 rounded-lg appearance-none cursor-pointer dark:bg-slate-600 accent-amber-500 dark:accent-teal-500" prop:value=move || start_energy().as_float().to_string() on:input=move |e| {
                            if let Ok(value) = event_target_value(&e).parse() {
                                if unplug_at.get_untracked().as_float() < value {
                                    set_unplug_at(PercentFull::new((value + 5.0).min(100.0)));
                                }
                                set_start_energy(PercentFull::new(value.min(95.0)));
                            }
                        }/>
                    </div>
                    <div class:collapse=move || vehicle_spec.with(|spec| spec.is_none()) class="flex flex-col">
                        <label for="battery-soc" class="block mb-2 text-sm font-medium text-slate-700 dark:text-slate-200">"Unplug at Battery SOC%: "{move || unplug_at().to_string()}" "{move || (unplug_at() * specs().battery_max).to_string()}</label>
                        <input id="battery-soc" type="range" class="w-full h-2 bg-amber-200 rounded-lg appearance-none cursor-pointer dark:bg-slate-600 accent-amber-500 dark:accent-teal-500" prop:value=move || unplug_at().as_float().to_string() on:input=move |e| {
                            if let Ok(value) = event_target_value(&e).parse() {
                                if start_energy.get_untracked().as_float() > value {
                                    set_start_energy(PercentFull::new((value - 5.0).max(0.0)));
                                }
                                set_unplug_at(PercentFull::new(value.max(5.0)));
                            }
                        }/>
                    </div>
                    <button class:collapse=move || vehicle_spec.with(|spec| spec.is_none()) class="bg-amber-500 dark:bg-teal-600 text-white p-2 border border-amber-600 dark:border-teal-500 hover:bg-amber-600 dark:hover:bg-teal-500 rounded-lg font-medium transition-colors"
                        on:click=move |_| {
                            if let Some(current) = vehicle_spec.get_untracked() {
                                let mut vehicles = vehicles();
                                vehicles.push_back(Vehicle::new(current, start_energy.get_untracked() * current.battery_max, unplug_at.get_untracked() * current.battery_max));
                                set_vehicles(vehicles);
                                set_vehicle_spec(None);
                            }
                        }>
                        "ADD +"
                    </button>
                </div>
                <ChargeCurve spec=vehicle_spec start_soc=start_energy end_soc=unplug_at />
            </div>
    }
}

#[component]
fn VehicleList(
    #[prop(into)] vehicles: Signal<VecDeque<Vehicle>>,
    set_vehicles: SignalSetter<VecDeque<Vehicle>>,
) -> impl IntoView {
    view! {
        <div class="grid grid-cols-4 gap-1" class:collapse=move || vehicles.with(|v| v.is_empty())>
            <h2 class="text-xl col-span-4">"Vehicles:"</h2>

            <For each={move || vehicles().into_iter().enumerate()}
                key=|(i, v)| (*i, v.spec_details().name)
                let:vehicle>
                <div class="col-span-2">{vehicle.1.spec.name.clone()}</div>
                <div>{vehicle.1.soc().to_string()}" -> "{vehicle.1.unplug_at_soc().to_string()}</div>
                <button class="hover:bg-red-500 bg-red-600 rounded-lg w-10 border border-red-700 text-white transition-colors" on:click=move |_| { let mut vehicles = vehicles(); vehicles.remove(vehicle.0); set_vehicles(vehicles); }>"X"</button>
            </For>

        </div>
    }
}

#[component]
fn ChargerBuilder(
    #[prop(into)] chargers: Signal<Vec<Charger>>,
    set_chargers: SignalSetter<Vec<Charger>>,
) -> impl IntoView {
    let (grid_connection, set_grid_connection) = signal(Power::from_kw(600.0));
    let load_share = RwSignal::new(LoadSharingStrategy::None);
    let number_of_plugs = Memo::new(move |_| match load_share.get() {
        LoadSharingStrategy::None => None,
        LoadSharingStrategy::Paired { number_of_plugs } => Some(number_of_plugs),
        LoadSharingStrategy::Split { number_of_plugs } => Some(number_of_plugs),
        LoadSharingStrategy::Granular { number_of_plugs, .. } => Some(number_of_plugs),
    });
    let set_number_of_plugs = move |plugs: u32| {
        load_share.update(|strategy| match strategy {
            LoadSharingStrategy::None => {}
            LoadSharingStrategy::Paired { number_of_plugs } => *number_of_plugs = plugs,
            LoadSharingStrategy::Split { number_of_plugs } => *number_of_plugs = plugs,
            LoadSharingStrategy::Granular { number_of_plugs, .. } => *number_of_plugs = plugs,
        });
    };
    let power_step = Memo::new(move |_| match load_share.get() {
        LoadSharingStrategy::Granular { power_step, .. } => Some(power_step),
        _ => None,
    });
    let set_power_step = move |step: Power| {
        load_share.update(|strategy| match strategy {
            LoadSharingStrategy::Granular { power_step, .. } => *power_step = step,
            _ => {}
        });
    };
    let max_per_plug = Memo::new(move |_| match load_share.get() {
        LoadSharingStrategy::Granular { max_per_plug, .. } => Some(max_per_plug),
        _ => None,
    });
    let set_max_per_plug = move |per_plug: Power| {
        load_share.update(|strategy| match strategy {
            LoadSharingStrategy::Granular { max_per_plug, .. } => *max_per_plug = per_plug,
            _ => {}
        });
    };
    let btn_active =
        "rounded-lg bg-amber-100 dark:bg-teal-900 p-1.5 border border-amber-400 dark:border-teal-500 font-medium";
    let btn_inactive = "rounded-lg bg-white dark:bg-slate-700 hover:bg-amber-50 dark:hover:bg-slate-600 p-1.5 border border-slate-300 dark:border-slate-500 transition-colors";
    view! {
        <div class="flex flex-col">
                <h4 class="text-xl">"Add Charger: "</h4>
                <div class="grid grid-cols-2">
                    <div>
                        "Grid Connection: "{move || grid_connection().to_string()}
                    </div>
                    <div>
                        <input class="dark:bg-slate-700 bg-white hover:bg-amber-50 dark:hover:bg-slate-600 border border-slate-300 dark:border-slate-500 rounded-lg p-1 w-36 transition-colors" value=grid_connection.get_untracked().as_kw() on:input=move |e| {
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
                            power_step: Power::from_kw(25.0),
                            max_per_plug: Power::from_kw(400.0),
                        })>"Granular"</button>
                        <div class="grid grid-cols-2 gap-1" class:collapse=move || number_of_plugs().is_none()>
                            <span>
                                "Number of plugs:"
                                {move || number_of_plugs().unwrap_or(1)}
                            </span>
                            <input class="dark:bg-slate-700 bg-white hover:bg-amber-50 dark:hover:bg-slate-600 border border-slate-300 dark:border-slate-500 rounded-lg p-1 w-36 shrink transition-colors" prop:value=move || number_of_plugs().unwrap_or_default() on:input=move |e| {
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
                            <input class="dark:bg-slate-700 bg-white hover:bg-amber-50 dark:hover:bg-slate-600 border border-slate-300 dark:border-slate-500 rounded-lg p-1 w-36 shrink transition-colors" prop:value=move || power_step().unwrap_or_default().as_kw() on:input=move |e| {
                                if let Ok(value) = event_target_value(&e).parse() {
                                    set_power_step(Power::from_kw(value));
                                }
                            }/>
                            <span>
                                "Max per plug:"
                                {move || max_per_plug().unwrap_or(Power::from_kw(1.0)).to_string()}
                            </span>
                            <input class="dark:bg-slate-700 bg-white hover:bg-amber-50 dark:hover:bg-slate-600 border border-slate-300 dark:border-slate-500 rounded-lg p-1 w-36 shrink transition-colors" prop:value=move || max_per_plug().unwrap_or_default().as_kw() on:input=move |e| {
                                if let Ok(value) = event_target_value(&e).parse() {
                                    set_max_per_plug(Power::from_kw(value));
                                }
                            }/>
                        </div>
                    </div>
                    <button class="bg-amber-500 dark:bg-teal-600 text-white hover:bg-amber-600 dark:hover:bg-teal-500 rounded-lg p-2 font-medium transition-colors" on:click=move |_| {
                        let strategy = load_share.get_untracked();
                        let mut chargers = chargers();
                        chargers.push(Charger::new(grid_connection.get_untracked(), strategy));
                        set_chargers(chargers);
                        load_share.set(LoadSharingStrategy::None);
                    }>"Add charger +"</button>
                </div>
            </div>
    }
}

#[component]
fn ChargerList(
    #[prop(into)] chargers: Signal<Vec<Charger>>,
    set_chargers: SignalSetter<Vec<Charger>>,
) -> impl IntoView {
    view! {
        <div class="grid grid-cols-2" class:collapse=move || chargers.with(|c| c.is_empty())>
            <h3 class="text-xl col-span-2">"Chargers: "</h3>
            <For each=move || chargers.get().into_iter().enumerate()
            key=|(i, c)| (*i, c.grid_connection.watts, format!("{:?}", c.strategy))
            let:charger>
            <div class="p-2 flex flex-row rounded-lg gap-1 bg-white dark:bg-slate-800 border-l-4 border-amber-400 dark:border-teal-500 shadow-sm">
                "Grid power: "{charger.1.grid_connection.to_string()}<br/>
                {match charger.1.strategy {
                    LoadSharingStrategy::None => "None".into_any(),
                    LoadSharingStrategy::Paired { number_of_plugs } => format!("Paired - {number_of_plugs}").into_any(),
                    LoadSharingStrategy::Split { number_of_plugs } => view!{ <div class="flex flex-col">
                        <span>"Even split"</span>
                        <span>"Number of plugs: "{number_of_plugs}</span></div>}.into_any(),
                    LoadSharingStrategy::Granular { power_step, number_of_plugs, max_per_plug } => view!{<div class="flex flex-col">
                        <span>"Incremental Share"</span>
                        <span>"Power step size: "{power_step.as_kw()}</span>
                        <span>"Number of plugs: "{number_of_plugs}</span>
                        <span>"Max per plug: "{max_per_plug.as_kw()}</span>
                    </div>}.into_any(),
                }}
            </div>
            <button class="hover:bg-red-500 bg-red-600 rounded-lg w-10 border border-red-700 text-white transition-colors" on:click=move |_| {
                let mut chargers = chargers();
                chargers.remove(charger.0);
                set_chargers(chargers);
            }>"X"</button>
            </For>
        </div>
    }
}

#[component]
fn ChargeCurve(
    #[prop(into)] spec: Signal<Option<&'static VehicleSpec>>,
    #[prop(into)] start_soc: Signal<PercentFull>,
    #[prop(into)] end_soc: Signal<PercentFull>,
) -> impl IntoView {
    let container = NodeRef::<html::Div>::new();
    let UseElementSizeReturn { width, height } = use_element_size(container);
    let dark_mode = use_preferred_dark();
    Effect::new(move |_| {
        let start_soc = start_soc();
        let end_soc = end_soc();
        let desired_width = (width() - 5.0).max(100.0) as u32;
        let desired_height = (height() - 5.0).max(100.0) as u32;

        if let Some(spec) = spec() {
            #[cfg(feature = "hydrate")]
            {
                use charming::{
                    component::{Axis, Legend, Title, VisualMap, VisualMapPiece, VisualMapType},
                    element::{
                        AreaStyle, AxisLabel, AxisType, Label, LineStyle, MarkLine, MarkLineData,
                        MarkLineVariant, NameLocation, Symbol,
                    },
                    series::Line,
                    WasmRenderer,
                };
                let points = spec
                    .charge_curve
                    .data_points
                    .iter()
                    .map(|point| vec![point.state_of_charge.as_float(), point.charge_power.as_kw()])
                    .collect::<Vec<_>>();
                let chart = charming::Chart::new()
                    .title(Title::new().text("Charging Curve"))
                    .x_axis(
                        Axis::new()
                            .name("Battery SOC%")
                            .name_location(NameLocation::Center)
                            .type_(AxisType::Value),
                    )
                    .y_axis(
                        Axis::new()
                            .name("Charge Power (KW)")
                            .type_(AxisType::Value)
                            .name_location(NameLocation::Center)
                            .name_gap(35.0)
                            .axis_label(AxisLabel::new().show(true)),
                    )
                    .visual_map(
                        VisualMap::new()
                            .type_(VisualMapType::Piecewise)
                            .show(false)
                            .dimension(0)
                            .series_index(0)
                            .pieces(vec![VisualMapPiece::new()
                                .gt(start_soc.as_float())
                                .lt(end_soc.as_float())])
                            .color(vec!["#5373BB70"]),
                    )
                    .series(
                        Line::new()
                            .name(spec.name)
                            .data(points)
                            .smooth(0.5)
                            .show_symbol(false)
                            .line_style(LineStyle::new().width(4.0).color("#5373BB"))
                            .mark_line(
                                MarkLine::new()
                                    .symbol(vec![Symbol::None, Symbol::None])
                                    .label(Label::new().show(false))
                                    .data(vec![
                                        MarkLineVariant::Simple(
                                            MarkLineData::new().x_axis(start_soc.as_float()),
                                        ),
                                        MarkLineVariant::Simple(
                                            MarkLineData::new().x_axis(end_soc.as_float()),
                                        ),
                                    ]),
                            )
                            .area_style(AreaStyle::new()),
                    )
                    .legend(Legend::new());
                let html = WasmRenderer::new(desired_width, desired_height);
                html.theme(match dark_mode() {
                    true => charming::theme::Theme::Dark,
                    false => charming::theme::Theme::Chalk,
                })
                .render("chargecurve", &chart)
                .unwrap();
            }
        }
    });
    view! {
        <Script src="https://cdn.jsdelivr.net/npm/echarts@5.4.2/dist/echarts.min.js"></Script>
        <Script src="https://cdn.jsdelivr.net/npm/echarts-gl@2.0.9/dist/echarts-gl.min.js"></Script>
        <div class:collapse=move || spec().is_none() class="flex flex-col">
            <div node_ref=container class="w-full h-screen md:h-[50vh]">{move || { let _ = dark_mode(); let _ = width(); let _ = height(); view!{ <div id="chargecurve"></div> }}}</div>
            <span>"Please note that the displayed curve may not be accurate."</span>
            <span>"Assumes charger can match voltage of the vehicle and optimal battery temperature."</span>
        </div>
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
struct ChargingVehicle {
    /// the power allocated by the charger to this vehicle currently
    allocated_power: Power,
    vehicle_id: usize,
    vehicle: Vehicle,
}

impl ChargingVehicle {
    fn summary(&self) -> VehicleChargeFrame {
        let allocated_power = self.allocated_power;
        VehicleChargeFrame {
            allocated_power,
            vehicle_id: self.vehicle_id,
        }
    }
}

trait IntDivCeil {
    /// divide and round up
    fn div_up(&self, other: Self) -> Self;
}

impl IntDivCeil for i32 {
    fn div_up(&self, other: Self) -> Self {
        let rem = self % other;
        self / other + (rem != 0) as i32
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
struct Charger {
    grid_connection: Power,
    strategy: LoadSharingStrategy,
    #[serde(skip)]
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

    fn add_vehicle(&mut self, vehicle: Vehicle, id: usize) {
        self.currently_charging.push(ChargingVehicle {
            allocated_power: Power::from_kw(0.0),
            vehicle,
            vehicle_id: id,
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

    fn total_allocated_power(&self) -> Power {
        self.currently_charging
            .iter()
            .map(|c| c.allocated_power)
            .sum()
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
                let total_steps = self.grid_connection.watts / power_step.watts;
                let mut power_steps_allocated = self
                    .currently_charging
                    .iter()
                    .map(|c| c.allocated_power.watts.div_up(power_step.watts))
                    .sum::<i32>();
                
                self.currently_charging.retain_mut(|c| {
                    let is_valid = (total_steps - power_steps_allocated > 0) as i32;
                    let available_power = c.allocated_power + (power_step * is_valid).min(max_per_plug);
                    if let Some(power) = c.vehicle.get_next_power_request(available_power) {
                        let old_power_steps = c.allocated_power.watts.div_up(power_step.watts);
                        let new_power_steps = power.watts.div_up(power_step.watts);
                        let next_power_steps = power_steps_allocated + new_power_steps - old_power_steps;
                        if total_steps < next_power_steps {
                            return true;
                        }
                        power_steps_allocated = next_power_steps;
                        c.allocated_power = power;
                        true
                    } else {
                        // return our power to the pool
                        power_steps_allocated += c.allocated_power.watts.div_up(power_step.watts);
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
    current_id: usize,
    /// all of the chargers in the simulation
    chargers: Vec<Charger>,
    /// length of time each step should simulate
    simulation_step_time: Duration,
    /// Total duration the simulation has simulated
    simulation_time: Duration,
}

#[derive(Clone, Copy)]
struct VehicleChargeFrame {
    allocated_power: Power,
    vehicle_id: usize,
}

#[derive(Copy, Clone)]
struct ChargerFrame {
    charger_id: usize,
    active_power: Power,
    unused_power: Power,
}

#[derive(Clone)]
struct SimFrame {
    energy_dispensed: Energy,
    chargers: Vec<ChargerFrame>,
    vehicles_charging: Vec<VehicleChargeFrame>,
    duration: Duration,
}

impl Sim {
    fn step(&mut self) -> SimFrame {
        // start charging any vehicles we can
        if !self.vehicles.is_empty() {
            for charger in self.chargers.iter_mut().filter(|c| c.has_free_plug()) {
                while charger.has_free_plug() && !self.vehicles.is_empty() {
                    charger.add_vehicle(self.vehicles.pop_front().unwrap(), self.current_id);
                    self.current_id += 1;
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
        let chargers = self
            .chargers
            .iter()
            .enumerate()
            .map(|(charger_id, charger)| {
                let active_power = charger.total_allocated_power();
                ChargerFrame {
                    charger_id,
                    active_power,
                    unused_power: (charger.grid_connection - active_power),
                }
            })
            .collect();
        SimFrame {
            energy_dispensed,
            vehicles_charging: self
                .chargers
                .iter()
                .flat_map(|c| c.currently_charging.iter().map(|c| c.summary()))
                .collect(),
            duration: self.simulation_time,
            chargers,
        }
    }

    fn is_valid(&self) -> bool {
        !self.chargers.is_empty() && !self.vehicles.is_empty()
    }

    fn is_done(&self) -> bool {
        self.vehicles.is_empty()
            && self
                .chargers
                .iter()
                .all(|c| c.currently_charging.is_empty())
    }
}

/// Represents the data from a single vehicle charging
struct SimVehicleSeriesData {
    spec: &'static VehicleSpec,
    id: usize,
    data: Vec<Vec<f64>>,
}

struct SimChargerSeriesData {
    id: usize,
    used_power: Vec<Vec<f64>>,
    unused_power: Vec<Vec<f64>>,
}

fn get_charge_data_from_vehicles(
    vehicles: Vec<&'static VehicleSpec>,
    data: &Vec<SimFrame>,
) -> (Vec<SimVehicleSeriesData>, Vec<SimChargerSeriesData>) {
    let mut vehicles = vehicles
        .into_iter()
        .enumerate()
        .map(|(id, spec)| SimVehicleSeriesData {
            spec,
            id,
            data: vec![],
        })
        .collect::<Vec<_>>();
    let mut chargers: Vec<_> = data
        .get(0)
        .map(|c| {
            c.chargers
                .iter()
                .enumerate()
                .map(|(i, _)| SimChargerSeriesData {
                    id: i,
                    used_power: vec![],
                    unused_power: vec![],
                })
                .collect()
        })
        .unwrap_or_default();
    for sim_frame in data {
        let time_mins = sim_frame.duration.as_secs_f64() / 60.0;
        for vehicle_frame in &sim_frame.vehicles_charging {
            vehicles[vehicle_frame.vehicle_id]
                .data
                .push(vec![time_mins, vehicle_frame.allocated_power.as_kw()]);
        }
        for charger in &sim_frame.chargers {
            chargers[charger.charger_id]
                .used_power
                .push(vec![time_mins, charger.active_power.as_kw()]);
            chargers[charger.charger_id]
                .unused_power
                .push(vec![time_mins, charger.unused_power.as_kw()]);
        }
    }
    (vehicles, chargers)
}

#[component]
fn SimulationChart(
    vehicles: Signal<Vec<&'static VehicleSpec>>,
    data: Signal<Vec<SimFrame>>,
    prefers_dark: Signal<bool>,
) -> impl IntoView {
    let node = NodeRef::<html::Div>::new();
    let UseElementSizeReturn { width, height } = use_element_size_with_options(
        node,
        UseElementSizeOptions::default().initial_size(Size {
            width: 1000.0,
            height: 500.0,
        }),
    );
    let is_large = use_media_query("(min-width: 728px)");
    Effect::new(move |_| {
        let desired_width = (width() - 10.0).max(100.0) as u32;
        let desired_height = (height() - 10.0).max(100.0) as u32;
        let vehicles = vehicles();
        let prefers_dark = prefers_dark();
        if data.with(|d| d.is_empty()) {
            return;
        }

        let (vehicle_curves, chargers) =
            data.with(|sim_data| get_charge_data_from_vehicles(vehicles, sim_data));
        #[cfg(feature = "hydrate")]
        {
            use charming::{
                component::{
                    Axis, Feature, Grid, Legend, LegendType, Restore, SaveAsImage, Title, Toolbox,
                    ToolboxDataZoom,
                },
                element::{AxisLabel, AxisType, NameLocation, Orient, Tooltip, Trigger, LineStyle, MarkArea},
                series::Line,
                WasmRenderer,
            };
            let mut chart = charming::Chart::new()
                .title(Title::new().text("Charging Simulation"))
                .grid(
                    Grid::new()
                        .right(if is_large() { 300 } else { 100 })
                        .left(62.0)
                        .top(62.0)
                        .bottom(50.0),
                )
                .x_axis(
                    Axis::new()
                        .name("Time elapsed (minutes)")
                        .name_location(NameLocation::Center)
                        .name_gap(25.0)
                        .axis_label(AxisLabel::new().show(true).formatter("{value} min"))
                        .type_(AxisType::Value),
                )
                .y_axis(
                    Axis::new()
                        .name("Charge Power (KW)")
                        .type_(AxisType::Value)
                        .name_location(NameLocation::Center)
                        .name_gap(50.0)
                        .axis_label(AxisLabel::new().formatter("{value} kw").show(true))
                        .boundary_gap(("0%", "0%")),
                )
                .y_axis(
                    Axis::new()
                        .name("Energy Dispensed (kwh)")
                        .type_(AxisType::Value)
                        .name_location(NameLocation::Center)
                        .name_gap(60.0)
                        .axis_label(AxisLabel::new().formatter("{value} kwH").show(true)),
                )
                .legend(if is_large() {
                    Legend::new()
                        .type_(LegendType::Scroll)
                        .orient(Orient::Vertical)
                        .right(0.0)
                        .top("center")
                } else {
                    Legend::new()
                        .type_(LegendType::Scroll)
                        .orient(Orient::Horizontal)
                        .top(25.0)
                })
                .tooltip(Tooltip::new().trigger(Trigger::Axis))
                .toolbox(
                    Toolbox::new().show(true).feature(
                        Feature::new()
                            .data_zoom(ToolboxDataZoom::new().y_axis_index("none"))
                            .restore(Restore::new())
                            .save_as_image(SaveAsImage::new().show(true)),
                    ),
                );
            for car in vehicle_curves {
                let SimVehicleSeriesData { spec, id, data } = car;
                chart = chart.series(
                    Line::new()
                        .name(format!("#{} {}", id + 1, spec.name))
                        .data(data)
                        .smooth(0.5)
                        .show_symbol(false),
                );
            }
            for charger in chargers {
                let SimChargerSeriesData {
                    id, unused_power, ..
                } = charger;
                chart = chart.series(
                    Line::new()
                        .name(format!("#{} charger unused power", id + 1))
                        .data(unused_power)
                        .smooth(0.5)
                        .mark_area(MarkArea::new())
                        .line_style(LineStyle::new().width(1.0))
                        .show_symbol(false),
                );
            }
            let energy_dispensed = data.with(|d| {
                let mut sum = Energy::from_kwh(0.0);
                d.iter()
                    .map(|s| {
                        sum += s.energy_dispensed;
                        vec![s.duration.as_secs_f64() / 60.0, sum.as_kwh()]
                    })
                    .collect::<Vec<_>>()
            });
            chart = chart.series(
                Line::new()
                    .name("Energy Dispensed (kwH)")
                    .data(energy_dispensed)
                    .smooth(0.5)
                    .show_symbol(false)
                    .y_axis_index(1),
            );

            let html = WasmRenderer::new(desired_width, desired_height);
            html.theme(match prefers_dark {
                true => charming::theme::Theme::Dark,
                false => charming::theme::Theme::Chalk,
            })
            .render("simchart", &chart)
            .unwrap();
        }
    });
    view! {<div class="w-full h-screen md:h-[750px]" node_ref=node>
        {move || {
            prefers_dark();
            let _ = width();
            let _ = height();
            view! { <div class:invisible=move || data.with(|d| d.is_empty()) id="simchart"></div> }
        }}
    </div>
    }
}

#[component]
fn Simulation(
    #[prop(into)] vehicles: Signal<VecDeque<Vehicle>>,
    #[prop(into)] chargers: Signal<Vec<Charger>>,
    sim_step: ReadSignal<Duration>,
) -> impl IntoView {
    let prefers_dark = leptos_use::use_preferred_dark();
    {
        move || {
            let v = vehicles();
            let (vehicles_signal, _) = signal(
                v.iter()
                    .flat_map(|v| v.spec.borrow().try_into())
                    .collect::<Vec<_>>(),
            );
            let c = chargers();
            let simulation_step_time = sim_step();
            let mut sim = Sim {
                vehicles: v,
                chargers: c,
                simulation_step_time,
                simulation_time: Duration::default(),
                current_id: 0,
            };
            let mut steps = vec![];
            if sim.is_valid() {
                while !sim.is_done() {
                    steps.push(sim.step());
                }
                let total_energy_dispensed =
                    steps.iter().map(|s| s.energy_dispensed).sum::<Energy>();
                let total_time_spent = steps.last().map(|s| s.duration).unwrap_or_default();
                let (steps_signal, _) = signal(steps.clone());
                view!{
                    <SimulationChart vehicles=vehicles_signal.into() data=steps_signal.into() prefers_dark />
                    <div class="flex flex-row flex-wrap gap-4 text-md">
                        <div>"energy dispensed: "{total_energy_dispensed.to_string()}</div>
                        <div>"minutes running: "{total_time_spent.as_secs()/60}</div>
                        <div>"vehicles: "{vehicles.with(|v| v.len())}</div>
                        <div>"chargers: "{chargers.with(|c| c.len())}</div>
                        <div>{simulation_step_time.as_secs().to_string()}" second simulation interval"</div>
                    </div>
                // <div class="grid grid-cols-4">
                //     <div>"minutes"</div><div>"energy dispensed"</div><div>"vehicles charging"</div><div>"plugs unused"</div>
                //     {steps.into_iter().enumerate().filter(|(i, _b)| i % 60 == 1).map(|(_, b)| b).map(|s| view!{
                //         <div>{format!("{:.2}", s.duration.as_secs_f64() / 60.0)}</div>
                //         <div>{s.energy_dispensed.to_string()}</div>
                //         <div>{s.vehicles_charging.len()}" total"
                //             <ul>
                //                 {s.vehicles_charging.into_iter().map(|summary| view!{ <li>{summary.allocated_power.to_string()}"@"{summary.soc.to_string()}", "{summary.spec.name.to_string()}</li>}).collect_view()}
                //             </ul>
                //         </div>
                //         <div>{s.plugs_unused}</div>
                //     }).collect_view()}
                // </div>
            }.into_any()
            } else {
                view! { "Add chargers and vehicles to get started" }.into_any()
            }
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq, Default, Clone)]
struct Query {
    chargers: Vec<Charger>,
    vehicles: VecDeque<Vehicle>,
}

fn create_compressed_query<T: DeserializeOwned + Serialize + PartialEq + Default + Send + Sync>(
) -> (Memo<T>, SignalSetter<T>) {
    let location = use_location();
    let navigate = use_navigate();
    let search = location.search;

    let get = Memo::new(move |_| {
        match search.with::<Result<_, Box<dyn std::error::Error>>>(|query_string| {
            let str = general_purpose::URL_SAFE.decode(query_string)?;
            let mut decompress_out = Vec::new();
            let mut decoder = DeflateDecoder::new(Cursor::new(str));
            let _end = decoder.read_to_end(&mut decompress_out)?;
            let query_string = String::from_utf8(decompress_out)?;
            Ok(serde_json::from_str::<T>(&query_string)?)
        }) {
            Ok(query) => query,
            Err(err) => {
                log::error!("Error reading {err}");
                T::default()
            }
        }
    });
    let set = SignalSetter::map(move |query: T| {
        let query = serde_json::to_string(&query).unwrap();
        let mut bytes = Vec::new();
        {
            let mut encoder = DeflateEncoder::new(&mut bytes, Compression::new(9));
            encoder.write_all(query.as_bytes()).unwrap();
            encoder.flush().unwrap();
        }
        let query = general_purpose::URL_SAFE.encode(bytes);
        let path = location.pathname.get_untracked();
        let hash = location.hash.get_untracked();
        let url = [path.as_str(), "?", query.as_str(), hash.as_str()].concat();
        navigate(&url, NavigateOptions::default());
    });
    (get, set)
}

fn create_sub_slice<T, S, F, F2, O>(
    getter: S,
    setter: SignalSetter<T>,
    get: F,
    get_mut: F2,
) -> (Memo<O>, SignalSetter<O>)
where
    S: Into<Signal<T>>,
    F: Fn(&T) -> &O + Send + Sync + 'static,
    F2: Fn(&mut T) -> &mut O + Send + Sync + 'static,
    O: PartialEq + Clone + Send + Sync,
    T: Clone + Send + Sync,
{
    let getter: Signal<T> = getter.into();
    let get = Memo::new(move |_| getter.with(|value| get(value).clone()));
    let set = SignalSetter::map(move |update| {
        let mut value = getter.get();
        *get_mut(&mut value) = update;
        setter.set(value)
    });
    (get, set)
}

#[component]
pub fn VehicleSim() -> impl IntoView {
    let (query, set_query) = create_compressed_query::<Query>();
    let (chargers, set_chargers) =
        create_sub_slice(query, set_query, |q| &q.chargers, |q| &mut q.chargers);
    let (vehicles, set_vehicles) =
        create_sub_slice(query, set_query, |q| &q.vehicles, |q| &mut q.vehicles);
    let (simulation_time, _) = signal(Duration::from_secs(1));
    view! {
        <Title text="DC Fast Charger Sim" />
        <div class="flex flex-col gap-2">
            <div class="flex flex-col gap-1">
                <h2 class="text-3xl font-bold gradient-text">"DC Fast Charging Simulator"</h2>
                <span>"Simulate real charging time for electric vehicles in the real world with a variety of fast chargers."</span>
            </div>
            <div class="flex flex-col gap-1">
                <Simulation vehicles chargers sim_step=simulation_time />
            </div>
            <div class="flex flex-col md:flex-row gap-1">
                <VehicleList vehicles set_vehicles/>
                <ChargerList chargers set_chargers />
            </div>
            <div class="flex flex-col gap-1">
                <VehicleChooser vehicles set_vehicles />
                <ChargerBuilder chargers set_chargers />
            </div>
        </div>
    }
}
