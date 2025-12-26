use fin_engine::{Housing, HousingMarket};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HousingBrowserProps {
    pub current_housing: Option<Housing>,
    pub market_id: String,
    pub current_cash: rust_decimal::Decimal,
    pub on_select_housing: Callback<Housing>,
    pub on_close: Callback<()>,
}

#[function_component(HousingBrowser)]
pub fn housing_browser(props: &HousingBrowserProps) -> Html {
    // Generate available housing options
    let available_housing = if props.market_id == "czech" {
        HousingMarket::generate_czech_housing()
    } else {
        Vec::new()
    };

    let on_close_click = {
        let on_close = props.on_close.clone();
        Callback::from(move |_| {
            on_close.emit(());
        })
    };

    html! {
        <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4 z-50">
            <div class="bg-white rounded-lg shadow-2xl max-w-4xl w-full max-h-[90vh] overflow-hidden">
                // Header
                <div class="bg-gradient-to-r from-teal-500 to-cyan-600 text-white p-6">
                    <div class="flex justify-between items-center">
                        <div>
                            <h2 class="text-2xl font-bold mb-1">{ "Housing Market" }</h2>
                            <p class="text-teal-100 text-sm">
                                { "Find your next place to live" }
                            </p>
                        </div>
                        <button
                            onclick={on_close_click}
                            class="text-white hover:bg-teal-600 rounded-full w-10 h-10 flex items-center justify-center transition"
                        >
                            { "✕" }
                        </button>
                    </div>
                </div>

                // Current Housing Section
                <div class="p-6 border-b border-gray-200">
                    <h3 class="text-lg font-semibold text-gray-800 mb-3">{ "Current Housing" }</h3>
                    {if let Some(housing) = &props.current_housing {
                        html! {
                            <div class="bg-green-50 border-2 border-green-500 rounded-lg p-4">
                                <div class="flex justify-between items-start">
                                    <div>
                                        <p class="text-xl font-bold text-gray-800">
                                            { housing.housing_type.name() }
                                        </p>
                                        <p class="text-sm text-gray-600 mb-1">
                                            { &housing.address }
                                            { " • " }
                                            { housing.location.name() }
                                        </p>
                                        <p class="text-xs text-gray-500">
                                            { format!("Rent: {:.0} Kč • Utilities: {:.0} Kč",
                                                housing.monthly_cost, housing.monthly_utilities) }
                                        </p>
                                    </div>
                                    <div class="text-right">
                                        <p class="text-2xl font-bold text-green-600">
                                            { format!("{:.0}", housing.total_monthly_cost()) }
                                            { " Kč" }
                                        </p>
                                        <p class="text-xs text-gray-500">{ "per month" }</p>
                                    </div>
                                </div>
                            </div>
                        }
                    } else {
                        html! {
                            <div class="bg-gray-50 rounded-lg p-4 text-center">
                                <p class="text-gray-500">{ "No housing selected" }</p>
                                <p class="text-sm text-gray-400 mt-1">{ "Choose your first place below" }</p>
                            </div>
                        }
                    }}
                </div>

                // Available Housing List
                <div class="p-6 overflow-y-auto max-h-96">
                    <h3 class="text-lg font-semibold text-gray-800 mb-4">{ "Available Housing" }</h3>

                    {if available_housing.is_empty() {
                        html! {
                            <div class="text-center py-8">
                                <p class="text-gray-500">{ "No housing available" }</p>
                            </div>
                        }
                    } else {
                        html! {
                            <div class="space-y-3">
                                {available_housing.iter().map(|housing| {
                                    let is_current = props.current_housing.as_ref()
                                        .map(|h| h.id == housing.id)
                                        .unwrap_or(false);

                                    let moving_cost = housing.moving_cost();
                                    let can_afford = props.current_cash >= moving_cost;

                                    let housing_clone = housing.clone();
                                    let on_select = {
                                        let on_select_housing = props.on_select_housing.clone();
                                        Callback::from(move |_| {
                                            on_select_housing.emit(housing_clone.clone());
                                        })
                                    };

                                    html! {
                                        <div
                                            key={housing.id.clone()}
                                            class={format!(
                                                "border-2 rounded-lg p-4 transition {}",
                                                if is_current {
                                                    "border-green-500 bg-green-50"
                                                } else if !can_afford {
                                                    "border-gray-200 bg-gray-50 opacity-60"
                                                } else {
                                                    "border-gray-200 hover:border-teal-300 hover:bg-teal-50"
                                                }
                                            )}
                                        >
                                            <div class="flex justify-between items-start mb-3">
                                                <div class="flex-1">
                                                    <div class="flex items-center gap-2 mb-1">
                                                        <p class="text-lg font-bold text-gray-800">
                                                            { housing.housing_type.name() }
                                                        </p>
                                                        <span class="text-xs px-2 py-1 rounded bg-cyan-100 text-cyan-700">
                                                            { housing.location.name() }
                                                        </span>
                                                    </div>
                                                    <p class="text-sm text-gray-600 mb-2">
                                                        { &housing.address }
                                                    </p>
                                                    <div class="text-xs text-gray-500 space-y-1">
                                                        <p>
                                                            { format!("Rent: {:.0} Kč/month", housing.monthly_cost) }
                                                        </p>
                                                        <p>
                                                            { format!("Utilities: {:.0} Kč/month", housing.monthly_utilities) }
                                                        </p>
                                                        <p class="font-semibold text-orange-600">
                                                            { format!("Moving cost: {:.0} Kč", moving_cost) }
                                                        </p>
                                                    </div>
                                                </div>
                                                <div class="text-right">
                                                    <p class="text-xl font-bold text-gray-800">
                                                        { format!("{:.0}", housing.total_monthly_cost()) }
                                                        { " Kč" }
                                                    </p>
                                                    <p class="text-xs text-gray-500">{ "per month" }</p>
                                                </div>
                                            </div>

                                            {if is_current {
                                                html! {
                                                    <button
                                                        class="w-full bg-green-500 text-white font-semibold py-2 px-4 rounded cursor-not-allowed opacity-50"
                                                        disabled=true
                                                    >
                                                        { "Current Home" }
                                                    </button>
                                                }
                                            } else if !can_afford {
                                                html! {
                                                    <button
                                                        class="w-full bg-gray-300 text-gray-600 font-semibold py-2 px-4 rounded cursor-not-allowed"
                                                        disabled=true
                                                    >
                                                        { format!("Cannot Afford (need {:.0} Kč)", moving_cost) }
                                                    </button>
                                                }
                                            } else {
                                                html! {
                                                    <button
                                                        onclick={on_select}
                                                        class="w-full bg-gradient-to-r from-teal-500 to-cyan-600 text-white font-semibold py-2 px-4 rounded hover:from-teal-600 hover:to-cyan-700 transition transform hover:scale-105"
                                                    >
                                                        {if props.current_housing.is_some() {
                                                            "Move Here"
                                                        } else {
                                                            "Select This Home"
                                                        }}
                                                    </button>
                                                }
                                            }}
                                        </div>
                                    }
                                }).collect::<Html>()}
                            </div>
                        }
                    }}
                </div>

                // Footer
                <div class="bg-gray-50 p-4 border-t border-gray-200">
                    <p class="text-xs text-gray-600 text-center">
                        { "Moving costs include security deposit (2 months rent) plus 1,500 Kč moving expenses" }
                    </p>
                </div>
            </div>
        </div>
    }
}
