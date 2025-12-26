use crate::components::{HousingBrowser, JobBrowser};
use fin_engine::{ExpenseCategory, GameState, Housing, Income, IncomeKind, Job};
use rust_decimal::Decimal;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PlanningProps {
    pub game_state: GameState,
    pub on_start_month: Callback<()>,
    pub on_update_state: Callback<GameState>,
}

#[function_component(PlanningScreen)]
pub fn planning_screen(props: &PlanningProps) -> Html {
    let game_state = &props.game_state;
    let finances = &game_state.finances;
    let player = &game_state.player;
    let career = &game_state.career;
    let housing = &game_state.housing;

    let net_worth = finances.net_worth();
    let monthly_income = finances.monthly_gross_income();
    let monthly_expenses = finances.monthly_expenses();
    let financial_peace = player.financial_peace_score();

    // Modal states
    let show_job_browser = use_state(|| false);
    let show_housing_browser = use_state(|| false);

    let on_start_click = {
        let on_start_month = props.on_start_month.clone();
        Callback::from(move |_| {
            on_start_month.emit(());
        })
    };

    let on_browse_jobs_click = {
        let show_job_browser = show_job_browser.clone();
        Callback::from(move |_| {
            show_job_browser.set(true);
        })
    };

    let on_close_job_browser = {
        let show_job_browser = show_job_browser.clone();
        Callback::from(move |_| {
            show_job_browser.set(false);
        })
    };

    let on_accept_job = {
        let show_job_browser = show_job_browser.clone();
        let on_update_state = props.on_update_state.clone();
        let game_state_clone = game_state.clone();
        Callback::from(move |job: Job| {
            // Clone the game state for modification
            let mut new_state = game_state_clone.clone();

            // If this is the first job, give starting cash and set minimum food budget
            let is_first_job = new_state.career.current_job.is_none();
            if is_first_job {
                // Give 50% of monthly salary as starting cash
                new_state.finances.cash = job.monthly_salary / Decimal::from(2);

                // Set minimum food budget (3,500 Kč/month - survival level)
                new_state
                    .finances
                    .set_budget(ExpenseCategory::Essential, Decimal::from(3500));
            }

            // Accept the job in career
            new_state.career.accept_job(job.clone());

            // Create or update income entry
            let income_id = format!("job_{}", job.id);

            // Remove any existing job income
            new_state
                .finances
                .income_sources
                .retain(|inc| !inc.id.starts_with("job_"));

            // Add new job income
            new_state.finances.income_sources.push(Income {
                id: income_id,
                name: job.title.clone(),
                kind: IncomeKind::Employment,
                gross_monthly: job.monthly_salary,
                active: true,
            });

            // Update state and close modal
            on_update_state.emit(new_state);
            show_job_browser.set(false);
        })
    };

    let on_browse_housing_click = {
        let show_housing_browser = show_housing_browser.clone();
        Callback::from(move |_| {
            show_housing_browser.set(true);
        })
    };

    let on_close_housing_browser = {
        let show_housing_browser = show_housing_browser.clone();
        Callback::from(move |_| {
            show_housing_browser.set(false);
        })
    };

    let on_select_housing = {
        let show_housing_browser = show_housing_browser.clone();
        let on_update_state = props.on_update_state.clone();
        let game_state_clone = game_state.clone();
        Callback::from(move |housing: Housing| {
            let mut new_state = game_state_clone.clone();

            // Try to change housing (handles affordability check and moving costs)
            match new_state.change_housing(housing) {
                Ok(_) => {
                    on_update_state.emit(new_state);
                    show_housing_browser.set(false);
                }
                Err(e) => {
                    // TODO: Show error message to user
                    web_sys::console::error_1(&format!("Cannot move: {}", e).into());
                }
            }
        })
    };

    // Budget allocation callbacks
    let on_budget_change = {
        let on_update_state = props.on_update_state.clone();
        let game_state_clone = game_state.clone();
        Callback::from(move |(category, amount): (ExpenseCategory, Decimal)| {
            let mut new_state = game_state_clone.clone();
            new_state.finances.set_budget(category, amount);
            on_update_state.emit(new_state);
        })
    };

    html! {
        <div class="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100">
            // Header
            <div class="bg-white shadow-md border-b border-gray-200">
                <div class="max-w-7xl mx-auto px-4 py-4">
                    <div class="flex justify-between items-center">
                        <div>
                            <h1 class="text-2xl font-bold text-gray-800">
                                { "Snowball" }
                            </h1>
                            <p class="text-sm text-gray-600">
                                { game_state.time.month.name() }
                                { " " }
                                { game_state.time.year }
                            </p>
                        </div>
                        <div class="flex gap-6">
                            <div class="text-right">
                                <p class="text-xs text-gray-500">{ "Net Worth" }</p>
                                <p class="text-lg font-bold text-gray-800">
                                    { format!("{:.2}", net_worth) }
                                    { " Kč" }
                                </p>
                            </div>
                            <div class="text-right">
                                <p class="text-xs text-gray-500">{ "Peace Score" }</p>
                                <p class="text-lg font-bold text-indigo-600">
                                    { financial_peace }
                                    { "/100" }
                                </p>
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            // Main Content
            <div class="max-w-4xl mx-auto px-4 py-8">
                // Phase Indicator
                <div class="bg-blue-500 text-white rounded-lg p-6 mb-6 shadow-lg">
                    <div class="flex items-center justify-between">
                        <div>
                            <h2 class="text-2xl font-bold mb-1">{ "Monthly Planning" }</h2>
                            <p class="text-blue-100">
                                { "Phase 1: Plan your budget and make financial decisions" }
                            </p>
                        </div>
                        <div class="bg-blue-400 rounded-full w-16 h-16 flex items-center justify-center">
                            <span class="text-3xl">{ "📋" }</span>
                        </div>
                    </div>
                </div>

                // Player Stats
                <div class="bg-white rounded-lg shadow-md p-6 mb-6">
                    <h3 class="text-lg font-semibold text-gray-800 mb-4">{ "Your Status" }</h3>
                    <div class="grid grid-cols-3 gap-4">
                        <div>
                            <p class="text-sm text-gray-600 mb-1">{ "Age" }</p>
                            <p class="text-2xl font-bold text-gray-800">{ player.age }</p>
                        </div>
                        <div>
                            <p class="text-sm text-gray-600 mb-1">{ "Happiness" }</p>
                            <div class="flex items-center gap-2">
                                <div class="flex-1 bg-gray-200 rounded-full h-3">
                                    <div
                                        class="bg-green-500 h-3 rounded-full transition-all"
                                        style={format!("width: {}%", player.happiness)}
                                    ></div>
                                </div>
                                <span class="text-sm font-semibold text-gray-700">
                                    { player.happiness }
                                </span>
                            </div>
                        </div>
                        <div>
                            <p class="text-sm text-gray-600 mb-1">{ "Burnout" }</p>
                            <div class="flex items-center gap-2">
                                <div class="flex-1 bg-gray-200 rounded-full h-3">
                                    <div
                                        class="bg-red-500 h-3 rounded-full transition-all"
                                        style={format!("width: {}%", player.burnout)}
                                    ></div>
                                </div>
                                <span class="text-sm font-semibold text-gray-700">
                                    { player.burnout }
                                </span>
                            </div>
                        </div>
                    </div>
                </div>

                // Career Section
                <div class="bg-white rounded-lg shadow-md p-6 mb-6">
                    <div class="flex justify-between items-center mb-4">
                        <h3 class="text-lg font-semibold text-gray-800">{ "Career" }</h3>
                        <button
                            onclick={on_browse_jobs_click}
                            class="bg-purple-500 hover:bg-purple-600 text-white font-semibold py-2 px-4 rounded transition"
                        >
                            { "Browse Jobs" }
                        </button>
                    </div>

                    {if let Some(job) = &career.current_job {
                        html! {
                            <div class="bg-green-50 border-2 border-green-500 rounded-lg p-4">
                                <div class="flex justify-between items-start">
                                    <div>
                                        <p class="text-xl font-bold text-gray-800">{ &job.title }</p>
                                        <p class="text-sm text-gray-600 mb-2">
                                            {if let Some(company) = &job.company {
                                                html! { <>{ company }{ " • " }</> }
                                            } else {
                                                html! {}
                                            }}
                                            { job.field.name() }
                                        </p>
                                        <div class="flex gap-4 text-xs text-gray-500">
                                            <span>
                                                { format!("{} years experience", career.years_experience) }
                                            </span>
                                            <span>
                                                { format!("{} months at position", career.months_in_current_job) }
                                            </span>
                                        </div>
                                    </div>
                                    <div class="text-right">
                                        <p class="text-2xl font-bold text-green-600">
                                            { format!("{:.0}", job.monthly_salary) }
                                            { " Kč" }
                                        </p>
                                        <p class="text-xs text-gray-500">{ "per month" }</p>
                                    </div>
                                </div>
                            </div>
                        }
                    } else {
                        html! {
                            <div class="bg-yellow-50 border-2 border-yellow-400 rounded-lg p-4 text-center">
                                <p class="text-yellow-800 font-semibold mb-1">{ "Currently Unemployed" }</p>
                                <p class="text-sm text-yellow-600">
                                    { "Click 'Browse Jobs' to find employment opportunities" }
                                </p>
                                <p class="text-xs text-gray-500 mt-2">
                                    { format!("Experience: {} years", career.years_experience) }
                                </p>
                            </div>
                        }
                    }}
                </div>

                // Housing Section
                <div class="bg-white rounded-lg shadow-md p-6 mb-6">
                    <div class="flex justify-between items-center mb-4">
                        <h3 class="text-lg font-semibold text-gray-800">{ "Housing" }</h3>
                        <button
                            onclick={on_browse_housing_click}
                            class="bg-teal-500 hover:bg-teal-600 text-white font-semibold py-2 px-4 rounded transition"
                        >
                            { "Browse Housing" }
                        </button>
                    </div>

                    {if let Some(home) = housing {
                        html! {
                            <div class="bg-teal-50 border-2 border-teal-500 rounded-lg p-4">
                                <div class="flex justify-between items-start">
                                    <div>
                                        <p class="text-xl font-bold text-gray-800">
                                            { home.housing_type.name() }
                                        </p>
                                        <p class="text-sm text-gray-600 mb-2">
                                            { &home.address }
                                            { " • " }
                                            { home.location.name() }
                                        </p>
                                        <div class="flex gap-4 text-xs text-gray-500">
                                            <span>
                                                { format!("Rent: {:.0} Kč", home.monthly_cost) }
                                            </span>
                                            <span>
                                                { format!("Utilities: {:.0} Kč", home.monthly_utilities) }
                                            </span>
                                            <span>
                                                { format!("{} months here", game_state.months_at_housing) }
                                            </span>
                                        </div>
                                    </div>
                                    <div class="text-right">
                                        <p class="text-2xl font-bold text-teal-600">
                                            { format!("{:.0}", home.total_monthly_cost()) }
                                            { " Kč" }
                                        </p>
                                        <p class="text-xs text-gray-500">{ "per month" }</p>
                                    </div>
                                </div>
                            </div>
                        }
                    } else {
                        html! {
                            <div class="bg-orange-50 border-2 border-orange-400 rounded-lg p-4 text-center">
                                <p class="text-orange-800 font-semibold mb-1">{ "No Housing Selected" }</p>
                                <p class="text-sm text-orange-600">
                                    { "Click 'Browse Housing' to find a place to live" }
                                </p>
                                <p class="text-xs text-gray-500 mt-2">
                                    { "Moving costs include security deposit + moving expenses" }
                                </p>
                            </div>
                        }
                    }}
                </div>

                // Budget Allocation Section
                <div class="bg-white rounded-lg shadow-md p-6 mb-6">
                    <h3 class="text-lg font-semibold text-gray-800 mb-4">{ "Monthly Budget Allocation" }</h3>
                    <p class="text-sm text-gray-600 mb-4">
                        { "Set your monthly budgets. Essential expenses have minimums you must meet." }
                    </p>

                    <div class="space-y-4">
                        // Essential Budget (Food & Groceries)
                        <div class="border-2 border-orange-300 bg-orange-50 rounded-lg p-4">
                            <div class="flex justify-between items-center mb-2">
                                <div>
                                    <p class="font-semibold text-gray-800">
                                        { "Food & Groceries " }
                                        <span class="text-red-600 text-xs">{ "(Required)" }</span>
                                    </p>
                                    <p class="text-xs text-gray-500">{ "Minimum: 3,500 Kč/month for survival" }</p>
                                </div>
                                {if let Some(budget) = finances.budget.get(&ExpenseCategory::Essential) {
                                    html! {
                                        <p class="text-sm text-gray-600">
                                            { format!("Spent: {:.0} / {:.0} Kč", budget.spent, budget.allocated) }
                                        </p>
                                    }
                                } else {
                                    html! {}
                                }}
                            </div>
                            <input
                                type="number"
                                min="3500"
                                class="w-full px-3 py-2 border border-orange-300 rounded focus:outline-none focus:ring-2 focus:ring-orange-500"
                                placeholder="3500"
                                value={finances.budget.get(&ExpenseCategory::Essential).map(|b| b.allocated.to_string()).unwrap_or("3500".to_string())}
                                oninput={
                                    let on_budget_change = on_budget_change.clone();
                                    Callback::from(move |e: InputEvent| {
                                        let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                        if let Ok(amount) = input.value().parse::<Decimal>() {
                                            // Enforce minimum of 3,500 Kč
                                            let final_amount = if amount < Decimal::from(3500) {
                                                Decimal::from(3500)
                                            } else {
                                                amount
                                            };
                                            on_budget_change.emit((ExpenseCategory::Essential, final_amount));
                                        }
                                    })
                                }
                            />
                            <p class="text-xs text-orange-700 mt-2">
                                { "This covers basic groceries. You can increase this for better food quality." }
                            </p>
                        </div>

                        // Discretionary Spending Header
                        <div class="pt-2 border-t-2 border-gray-200">
                            <p class="text-sm font-semibold text-gray-700 mb-3">{ "Discretionary Spending (Optional)" }</p>
                        </div>

                        // Lifestyle Budget
                        <div class="border border-gray-200 rounded-lg p-4">
                            <div class="flex justify-between items-center mb-2">
                                <div>
                                    <p class="font-semibold text-gray-800">{ "Lifestyle & Entertainment" }</p>
                                    <p class="text-xs text-gray-500">{ "Dining out, hobbies, entertainment" }</p>
                                </div>
                                {if let Some(budget) = finances.budget.get(&ExpenseCategory::Lifestyle) {
                                    html! {
                                        <p class="text-sm text-gray-600">
                                            { format!("Spent: {:.0} / {:.0} Kč", budget.spent, budget.allocated) }
                                        </p>
                                    }
                                } else {
                                    html! {}
                                }}
                            </div>
                            <input
                                type="number"
                                class="w-full px-3 py-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-purple-500"
                                placeholder="0"
                                value={finances.budget.get(&ExpenseCategory::Lifestyle).map(|b| b.allocated.to_string()).unwrap_or_default()}
                                oninput={
                                    let on_budget_change = on_budget_change.clone();
                                    Callback::from(move |e: InputEvent| {
                                        let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                        if let Ok(amount) = input.value().parse::<Decimal>() {
                                            on_budget_change.emit((ExpenseCategory::Lifestyle, amount));
                                        }
                                    })
                                }
                            />
                        </div>

                        // Health & Wellness Budget
                        <div class="border border-gray-200 rounded-lg p-4">
                            <div class="flex justify-between items-center mb-2">
                                <div>
                                    <p class="font-semibold text-gray-800">{ "Health & Wellness" }</p>
                                    <p class="text-xs text-gray-500">{ "Gym, sports, wellness activities" }</p>
                                </div>
                                {if let Some(budget) = finances.budget.get(&ExpenseCategory::Health) {
                                    html! {
                                        <p class="text-sm text-gray-600">
                                            { format!("Spent: {:.0} / {:.0} Kč", budget.spent, budget.allocated) }
                                        </p>
                                    }
                                } else {
                                    html! {}
                                }}
                            </div>
                            <input
                                type="number"
                                class="w-full px-3 py-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-purple-500"
                                placeholder="0"
                                value={finances.budget.get(&ExpenseCategory::Health).map(|b| b.allocated.to_string()).unwrap_or_default()}
                                oninput={
                                    let on_budget_change = on_budget_change.clone();
                                    Callback::from(move |e: InputEvent| {
                                        let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                        if let Ok(amount) = input.value().parse::<Decimal>() {
                                            on_budget_change.emit((ExpenseCategory::Health, amount));
                                        }
                                    })
                                }
                            />
                        </div>

                        // Transportation Budget
                        <div class="border border-gray-200 rounded-lg p-4">
                            <div class="flex justify-between items-center mb-2">
                                <div>
                                    <p class="font-semibold text-gray-800">{ "Transportation" }</p>
                                    <p class="text-xs text-gray-500">{ "Public transit, gas, rideshares" }</p>
                                </div>
                                {if let Some(budget) = finances.budget.get(&ExpenseCategory::Transportation) {
                                    html! {
                                        <p class="text-sm text-gray-600">
                                            { format!("Spent: {:.0} / {:.0} Kč", budget.spent, budget.allocated) }
                                        </p>
                                    }
                                } else {
                                    html! {}
                                }}
                            </div>
                            <input
                                type="number"
                                class="w-full px-3 py-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-purple-500"
                                placeholder="0"
                                value={finances.budget.get(&ExpenseCategory::Transportation).map(|b| b.allocated.to_string()).unwrap_or_default()}
                                oninput={
                                    let on_budget_change = on_budget_change.clone();
                                    Callback::from(move |e: InputEvent| {
                                        let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                        if let Ok(amount) = input.value().parse::<Decimal>() {
                                            on_budget_change.emit((ExpenseCategory::Transportation, amount));
                                        }
                                    })
                                }
                            />
                        </div>

                        // Education Budget
                        <div class="border border-gray-200 rounded-lg p-4">
                            <div class="flex justify-between items-center mb-2">
                                <div>
                                    <p class="font-semibold text-gray-800">{ "Education & Development" }</p>
                                    <p class="text-xs text-gray-500">{ "Courses, books, skill development" }</p>
                                </div>
                                {if let Some(budget) = finances.budget.get(&ExpenseCategory::Education) {
                                    html! {
                                        <p class="text-sm text-gray-600">
                                            { format!("Spent: {:.0} / {:.0} Kč", budget.spent, budget.allocated) }
                                        </p>
                                    }
                                } else {
                                    html! {}
                                }}
                            </div>
                            <input
                                type="number"
                                class="w-full px-3 py-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-purple-500"
                                placeholder="0"
                                value={finances.budget.get(&ExpenseCategory::Education).map(|b| b.allocated.to_string()).unwrap_or_default()}
                                oninput={
                                    let on_budget_change = on_budget_change.clone();
                                    Callback::from(move |e: InputEvent| {
                                        let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                        if let Ok(amount) = input.value().parse::<Decimal>() {
                                            on_budget_change.emit((ExpenseCategory::Education, amount));
                                        }
                                    })
                                }
                            />
                        </div>

                        // Other Budget
                        <div class="border border-gray-200 rounded-lg p-4">
                            <div class="flex justify-between items-center mb-2">
                                <div>
                                    <p class="font-semibold text-gray-800">{ "Other Expenses" }</p>
                                    <p class="text-xs text-gray-500">{ "Miscellaneous spending" }</p>
                                </div>
                                {if let Some(budget) = finances.budget.get(&ExpenseCategory::Other) {
                                    html! {
                                        <p class="text-sm text-gray-600">
                                            { format!("Spent: {:.0} / {:.0} Kč", budget.spent, budget.allocated) }
                                        </p>
                                    }
                                } else {
                                    html! {}
                                }}
                            </div>
                            <input
                                type="number"
                                class="w-full px-3 py-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-purple-500"
                                placeholder="0"
                                value={finances.budget.get(&ExpenseCategory::Other).map(|b| b.allocated.to_string()).unwrap_or_default()}
                                oninput={
                                    let on_budget_change = on_budget_change.clone();
                                    Callback::from(move |e: InputEvent| {
                                        let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                        if let Ok(amount) = input.value().parse::<Decimal>() {
                                            on_budget_change.emit((ExpenseCategory::Other, amount));
                                        }
                                    })
                                }
                            />
                        </div>

                        // Total Budget Summary
                        <div class="border-t-2 border-gray-300 pt-4 mt-2">
                            <div class="flex justify-between items-center">
                                <p class="text-lg font-semibold text-gray-800">{ "Total Monthly Budget" }</p>
                                <p class="text-xl font-bold text-purple-600">
                                    {{
                                        let total: Decimal = finances.budget.values()
                                            .map(|b| b.allocated)
                                            .sum();
                                        format!("{:.0} Kč", total)
                                    }}
                                </p>
                            </div>
                        </div>
                    </div>
                </div>

                // Financial Overview
                <div class="bg-white rounded-lg shadow-md p-6 mb-6">
                    <h3 class="text-lg font-semibold text-gray-800 mb-4">{ "Financial Overview" }</h3>
                    <div class="space-y-4">
                        <div class="flex justify-between items-center pb-3 border-b border-gray-200">
                            <span class="text-gray-600">{ "Monthly Income (Gross)" }</span>
                            <span class="text-lg font-bold text-green-600">
                                {if monthly_income > Decimal::ZERO {
                                    html! { <>{ format!("{:.2}", monthly_income) }{ " Kč" }</> }
                                } else {
                                    html! { <span class="text-gray-400">{ "No income yet" }</span> }
                                }}
                            </span>
                        </div>
                        <div class="flex justify-between items-center pb-3 border-b border-gray-200">
                            <span class="text-gray-600">{ "Monthly Expenses" }</span>
                            <span class="text-lg font-bold text-red-600">
                                {if monthly_expenses > Decimal::ZERO {
                                    html! { <>{ format!("{:.2}", monthly_expenses) }{ " Kč" }</> }
                                } else {
                                    html! { <span class="text-gray-400">{ "No expenses yet" }</span> }
                                }}
                            </span>
                        </div>

                        // Essential expenses breakdown
                        {if !finances.expenses.is_empty() || finances.budget.contains_key(&ExpenseCategory::Essential) {
                            html! {
                                <div class="pl-4 pb-3 border-b border-gray-200">
                                    <p class="text-xs text-gray-500 mb-2">{ "Essential Expenses:" }</p>

                                    // Food budget (if set)
                                    {if let Some(food_budget) = finances.budget.get(&ExpenseCategory::Essential) {
                                        html! {
                                            <div class="flex justify-between items-center text-sm mb-1">
                                                <span class="text-gray-600">{ "Food & Groceries" }</span>
                                                <span class="text-gray-700">
                                                    { format!("{:.0} Kč", food_budget.allocated) }
                                                </span>
                                            </div>
                                        }
                                    } else {
                                        html! {}
                                    }}

                                    // Fixed expenses (housing, etc.)
                                    {finances.expenses.iter()
                                        .filter(|e| e.category.is_essential() && e.active)
                                        .map(|expense| {
                                            html! {
                                                <div class="flex justify-between items-center text-sm mb-1">
                                                    <span class="text-gray-600">{ &expense.name }</span>
                                                    <span class="text-gray-700">
                                                        { format!("{:.0} Kč", expense.monthly_amount) }
                                                    </span>
                                                </div>
                                            }
                                        })
                                        .collect::<Html>()}
                                </div>
                            }
                        } else {
                            html! {}
                        }}

                        <div class="flex justify-between items-center">
                            <span class="text-gray-600 font-semibold">{ "Cash Balance" }</span>
                            <span class="text-xl font-bold text-gray-800">
                                { format!("{:.2}", finances.cash) }
                                { " Kč" }
                            </span>
                        </div>
                    </div>
                </div>

                // Getting Started Info
                {if monthly_income == Decimal::ZERO {
                    html! {
                        <div class="bg-yellow-50 border-l-4 border-yellow-400 p-6 mb-6">
                            <div class="flex">
                                <div class="flex-shrink-0">
                                    <span class="text-2xl">{ "💡" }</span>
                                </div>
                                <div class="ml-3">
                                    <h3 class="text-sm font-semibold text-yellow-800 mb-2">
                                        { "Getting Started" }
                                    </h3>
                                    <p class="text-sm text-yellow-700 mb-2">
                                        { "You're just starting out! In a real game, you would:" }
                                    </p>
                                    <ul class="text-sm text-yellow-700 list-disc list-inside space-y-1">
                                        <li>{ "Set up your income sources (job, freelance, etc.)" }</li>
                                        <li>{ "Add your regular expenses (rent, food, utilities)" }</li>
                                        <li>{ "Allocate budget for different spending categories" }</li>
                                        <li>{ "Plan investments and savings" }</li>
                                    </ul>
                                    <p class="text-sm text-yellow-700 mt-2">
                                        { "For now, click below to see how the month execution works!" }
                                    </p>
                                </div>
                            </div>
                        </div>
                    }
                } else {
                    html! {}
                }}

                // Action Button
                <div class="flex justify-center">
                    <button
                        onclick={on_start_click}
                        class="bg-gradient-to-r from-blue-500 to-indigo-600 text-white font-bold py-4 px-8 rounded-lg hover:from-blue-600 hover:to-indigo-700 transform transition hover:scale-105 shadow-lg text-lg"
                    >
                        { "Start Month →" }
                    </button>
                </div>
            </div>

            // Job Browser Modal
            {if *show_job_browser {
                html! {
                    <JobBrowser
                        career={career.clone()}
                        market_id={game_state.market_id.clone()}
                        on_accept_job={on_accept_job}
                        on_close={on_close_job_browser}
                    />
                }
            } else {
                html! {}
            }}

            // Housing Browser Modal
            {if *show_housing_browser {
                html! {
                    <HousingBrowser
                        current_housing={housing.clone()}
                        market_id={game_state.market_id.clone()}
                        current_cash={finances.cash}
                        on_select_housing={on_select_housing}
                        on_close={on_close_housing_browser}
                    />
                }
            } else {
                html! {}
            }}
        </div>
    }
}
