use fin_engine::{Job, JobMarket};
use yew::prelude::*;

/// Market option for selection
#[derive(Debug, Clone, PartialEq)]
pub struct MarketOption {
    pub id: &'static str,
    pub name: &'static str,
    pub currency_symbol: &'static str,
    pub available: bool,
}

const MARKET_OPTIONS: &[MarketOption] = &[
    MarketOption {
        id: "czech",
        name: "Czech Republic",
        currency_symbol: "Kč",
        available: true,
    },
    MarketOption {
        id: "usa",
        name: "United States",
        currency_symbol: "$",
        available: false,
    },
    MarketOption {
        id: "uk",
        name: "United Kingdom",
        currency_symbol: "£",
        available: false,
    },
];

#[derive(Properties, PartialEq)]
pub struct InitializationProps {
    pub on_start: Callback<InitializationData>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InitializationData {
    pub player_name: Option<String>,
    pub player_age: u8,
    pub market_id: String,
    pub starting_job: Option<Job>,
}

#[function_component(Initialization)]
pub fn initialization(props: &InitializationProps) -> Html {
    let player_name = use_state(String::new);
    let player_age = use_state(|| 25u8);
    let selected_market = use_state(|| "czech".to_string());
    let selected_job = use_state(|| Option::<Job>::None);
    let validation_error = use_state(|| Option::<String>::None);

    // Generate entry-level jobs for the selected market
    let available_jobs = use_memo((*selected_market).clone(), |market_id| {
        // Create a dummy career with 0 experience to get entry-level jobs
        let dummy_career = fin_engine::Career::new();
        if market_id == "czech" {
            JobMarket::generate_czech_jobs(&dummy_career)
                .into_iter()
                .filter(|job| job.required_experience == 0)
                .collect::<Vec<Job>>()
        } else {
            Vec::new()
        }
    });

    let on_name_change = {
        let player_name = player_name.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            player_name.set(input.value());
        })
    };

    let on_age_change = {
        let player_age = player_age.clone();
        let validation_error = validation_error.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            if let Ok(age) = input.value().parse::<u8>() {
                if (18..=65).contains(&age) {
                    player_age.set(age);
                    validation_error.set(None);
                } else {
                    validation_error.set(Some("Age must be between 18 and 65".to_string()));
                }
            }
        })
    };

    let on_market_select = {
        let selected_market = selected_market.clone();
        let selected_job = selected_job.clone();
        Callback::from(move |market_id: String| {
            selected_market.set(market_id);
            // Reset job selection when market changes
            selected_job.set(None);
        })
    };

    let on_job_select = {
        let selected_job = selected_job.clone();
        Callback::from(move |job: Job| {
            selected_job.set(Some(job));
        })
    };

    let on_submit = {
        let player_name = player_name.clone();
        let player_age = player_age.clone();
        let selected_market = selected_market.clone();
        let selected_job = selected_job.clone();
        let validation_error = validation_error.clone();
        let on_start = props.on_start.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let age = *player_age;
            if !(18..=65).contains(&age) {
                validation_error.set(Some("Age must be between 18 and 65".to_string()));
                return;
            }

            let name = if player_name.trim().is_empty() {
                None
            } else {
                Some(player_name.trim().to_string())
            };

            let data = InitializationData {
                player_name: name,
                player_age: age,
                market_id: (*selected_market).clone(),
                starting_job: (*selected_job).clone(),
            };

            on_start.emit(data);
        })
    };

    html! {
        <div class="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100 flex items-center justify-center p-4">
            <div class="bg-white rounded-2xl shadow-2xl p-8 max-w-2xl w-full">
                <div class="text-center mb-8">
                    <h1 class="text-5xl font-bold text-gray-800 mb-2">
                        { "Snowball" }
                    </h1>
                    <p class="text-xl text-gray-600">
                        { "Financial Education Game" }
                    </p>
                </div>

                <div class="bg-blue-50 border-l-4 border-blue-500 p-4 mb-6">
                    <p class="text-blue-800 text-sm">
                        { "Learn financial literacy through simulation. Manage your life through monthly cycles, \
                           master budgeting, investing, and achieve Financial Independence." }
                    </p>
                </div>

                <form onsubmit={on_submit}>
                    // Player Name
                    <div class="mb-6">
                        <label class="block text-gray-700 text-sm font-semibold mb-2">
                            { "Your Name (Optional)" }
                        </label>
                        <input
                            type="text"
                            class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                            placeholder="Enter your name"
                            value={(*player_name).clone()}
                            oninput={on_name_change}
                        />
                    </div>

                    // Player Age
                    <div class="mb-6">
                        <label class="block text-gray-700 text-sm font-semibold mb-2">
                            { "Starting Age" }
                        </label>
                        <input
                            type="number"
                            min="18"
                            max="65"
                            class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                            value={player_age.to_string()}
                            oninput={on_age_change}
                        />
                        <p class="text-gray-500 text-xs mt-1">
                            { "Choose your starting age (18-65)" }
                        </p>
                    </div>

                    // Market Selection
                    <div class="mb-6">
                        <label class="block text-gray-700 text-sm font-semibold mb-3">
                            { "Select Financial Market" }
                        </label>
                        <div class="space-y-3">
                            {MARKET_OPTIONS.iter().map(|market| {
                                let is_selected = *selected_market == market.id;
                                let market_id = market.id.to_string();
                                let on_click = {
                                    let on_market_select = on_market_select.clone();
                                    let market_id = market_id.clone();
                                    Callback::from(move |_| {
                                        on_market_select.emit(market_id.clone());
                                    })
                                };

                                let card_class = if market.available {
                                    if is_selected {
                                        "border-2 border-blue-500 bg-blue-50 cursor-pointer"
                                    } else {
                                        "border-2 border-gray-200 hover:border-blue-300 cursor-pointer"
                                    }
                                } else {
                                    "border-2 border-gray-200 bg-gray-50 opacity-50 cursor-not-allowed"
                                };

                                html! {
                                    <div
                                        key={market.id}
                                        class={format!("p-4 rounded-lg transition-all {}", card_class)}
                                        onclick={if market.available { on_click } else { Callback::from(|_| {}) }}
                                    >
                                        <div class="flex items-center justify-between">
                                            <div class="flex items-center space-x-3">
                                                <div class={format!(
                                                    "w-5 h-5 rounded-full border-2 flex items-center justify-center {}",
                                                    if is_selected { "border-blue-500 bg-blue-500" } else { "border-gray-300" }
                                                )}>
                                                    {if is_selected {
                                                        html! { <div class="w-2 h-2 bg-white rounded-full"></div> }
                                                    } else {
                                                        html! {}
                                                    }}
                                                </div>
                                                <div>
                                                    <div class="font-semibold text-gray-800">
                                                        { market.name }
                                                        {if !market.available {
                                                            html! { <span class="text-xs text-gray-500 ml-2">{ "(Coming Soon)" }</span> }
                                                        } else {
                                                            html! {}
                                                        }}
                                                    </div>
                                                    <div class="text-sm text-gray-600">
                                                        { "Currency: " }
                                                        { market.currency_symbol }
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                }
                            }).collect::<Html>()}
                        </div>
                    </div>

                    // Starting Job Selection
                    <div class="mb-6">
                        <label class="block text-gray-700 text-sm font-semibold mb-3">
                            { "Choose Your Starting Job " }
                            <span class="text-gray-500 font-normal">{ "(Optional)" }</span>
                        </label>
                        {if available_jobs.is_empty() {
                            html! {
                                <div class="p-4 bg-gray-50 rounded-lg text-center text-gray-500 text-sm">
                                    { "Select a market to see available entry-level jobs" }
                                </div>
                            }
                        } else {
                            html! {
                                <div class="space-y-2 max-h-64 overflow-y-auto">
                                    {available_jobs.iter().map(|job| {
                                        let is_selected = selected_job.as_ref().map(|j| j.id == job.id).unwrap_or(false);
                                        let job_clone = job.clone();
                                        let on_click = {
                                            let on_job_select = on_job_select.clone();
                                            Callback::from(move |_| {
                                                on_job_select.emit(job_clone.clone());
                                            })
                                        };

                                        let card_class = if is_selected {
                                            "border-2 border-purple-500 bg-purple-50 cursor-pointer"
                                        } else {
                                            "border-2 border-gray-200 hover:border-purple-300 cursor-pointer"
                                        };

                                        html! {
                                            <div
                                                key={job.id.clone()}
                                                class={format!("p-3 rounded-lg transition-all {}", card_class)}
                                                onclick={on_click}
                                            >
                                                <div class="flex justify-between items-start">
                                                    <div class="flex items-start space-x-2 flex-1">
                                                        <div class={format!(
                                                            "w-4 h-4 mt-0.5 rounded-full border-2 flex items-center justify-center flex-shrink-0 {}",
                                                            if is_selected { "border-purple-500 bg-purple-500" } else { "border-gray-300" }
                                                        )}>
                                                            {if is_selected {
                                                                html! { <div class="w-2 h-2 bg-white rounded-full"></div> }
                                                            } else {
                                                                html! {}
                                                            }}
                                                        </div>
                                                        <div class="flex-1">
                                                            <div class="font-semibold text-gray-800 text-sm">
                                                                { &job.title }
                                                            </div>
                                                            <div class="text-xs text-gray-600">
                                                                {if let Some(company) = &job.company {
                                                                    html! { <>{ company }{ " • " }</> }
                                                                } else {
                                                                    html! {}
                                                                }}
                                                                { job.field.name() }
                                                            </div>
                                                        </div>
                                                    </div>
                                                    <div class="text-right ml-2">
                                                        <div class="text-sm font-bold text-gray-800">
                                                            { format!("{:.0} Kč", job.monthly_salary) }
                                                        </div>
                                                        <div class="text-xs text-gray-500">
                                                            { "per month" }
                                                        </div>
                                                    </div>
                                                </div>
                                            </div>
                                        }
                                    }).collect::<Html>()}
                                </div>
                            }
                        }}
                        <p class="text-gray-500 text-xs mt-2">
                            { "You can start unemployed and look for jobs later, or choose an entry-level position now" }
                        </p>
                    </div>

                    // Validation Error
                    {if let Some(error) = (*validation_error).as_ref() {
                        html! {
                            <div class="mb-4 p-3 bg-red-50 border border-red-200 rounded-lg">
                                <p class="text-red-700 text-sm">{ error }</p>
                            </div>
                        }
                    } else {
                        html! {}
                    }}

                    // Submit Button
                    <button
                        type="submit"
                        class="w-full bg-gradient-to-r from-blue-500 to-indigo-600 text-white font-bold py-4 px-6 rounded-lg hover:from-blue-600 hover:to-indigo-700 transform transition hover:scale-105 shadow-lg"
                    >
                        { "Start Your Journey" }
                    </button>
                </form>

                <div class="mt-6 text-center text-xs text-gray-500">
                    { "All game data is stored locally in your browser" }
                </div>
            </div>
        </div>
    }
}
