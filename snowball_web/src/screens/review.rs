use fin_engine::{CzechMarket, GameState};
use rust_decimal::Decimal;
use yew::prelude::*;

fn get_market_profile(market_id: &str) -> Box<dyn fin_engine::market::MarketProfile> {
    match market_id {
        "czech" => Box::new(CzechMarket),
        _ => Box::new(CzechMarket),
    }
}

#[derive(Properties, PartialEq)]
pub struct ReviewProps {
    pub game_state: GameState,
    pub on_next_month: Callback<()>,
}

#[function_component(ReviewScreen)]
pub fn review_screen(props: &ReviewProps) -> Html {
    let game_state = &props.game_state;
    let player = &game_state.player;
    let finances = &game_state.finances;

    let net_worth = finances.net_worth();
    let financial_peace = player.financial_peace_score();
    let months_elapsed = game_state.months_elapsed();

    // Calculate monthly cash flow breakdown
    let gross_income = finances.monthly_gross_income();
    let market = get_market_profile(&game_state.market_id);
    let (net_income, total_tax) = if gross_income > Decimal::ZERO {
        if let Ok(tax_breakdown) = market.calculate_income_tax(gross_income) {
            (gross_income - tax_breakdown.total, tax_breakdown.total)
        } else {
            (gross_income, Decimal::ZERO)
        }
    } else {
        (Decimal::ZERO, Decimal::ZERO)
    };
    let total_expenses = finances.monthly_expenses();
    let net_cash_flow = net_income - total_expenses;

    let on_continue = {
        let on_next_month = props.on_next_month.clone();
        Callback::from(move |_| {
            on_next_month.emit(());
        })
    };

    html! {
        <div class="min-h-screen bg-gradient-to-br from-green-50 to-emerald-100">
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
                                { " - Complete" }
                            </p>
                        </div>
                        <div class="text-right">
                            <p class="text-xs text-gray-500">{ "Months Played" }</p>
                            <p class="text-lg font-bold text-gray-800">
                                { months_elapsed }
                            </p>
                        </div>
                    </div>
                </div>
            </div>

            // Main Content
            <div class="max-w-4xl mx-auto px-4 py-8">
                // Phase Indicator
                <div class="bg-green-500 text-white rounded-lg p-6 mb-6 shadow-lg">
                    <div class="flex items-center justify-between">
                        <div>
                            <h2 class="text-2xl font-bold mb-1">{ "Monthly Review" }</h2>
                            <p class="text-green-100">
                                { "Phase 3: Review your progress and prepare for next month" }
                            </p>
                        </div>
                        <div class="bg-green-400 rounded-full w-16 h-16 flex items-center justify-center">
                            <span class="text-3xl">{ "📊" }</span>
                        </div>
                    </div>
                </div>

                // Success Message
                <div class="bg-white rounded-lg shadow-md p-8 mb-6 text-center">
                    <div class="mb-4">
                        <span class="text-6xl">{ "🎉" }</span>
                    </div>
                    <h3 class="text-3xl font-bold text-gray-800 mb-2">
                        { "Month Complete!" }
                    </h3>
                    <p class="text-gray-600 mb-4">
                        { "You've successfully completed " }
                        { game_state.time.month.name() }
                        { ". Here's how you did:" }
                    </p>
                </div>

                // Monthly Cash Flow
                <div class="bg-white rounded-lg shadow-md p-6 mb-6">
                    <h3 class="text-lg font-semibold text-gray-800 mb-4">{ "Monthly Cash Flow" }</h3>
                    <div class="space-y-3">
                        <div class="flex justify-between items-center pb-2 border-b border-gray-200">
                            <span class="text-sm text-gray-600">{ "Gross Income" }</span>
                            <span class="text-lg font-semibold text-gray-800">
                                { format!("+{:.0} Kč", gross_income) }
                            </span>
                        </div>
                        <div class="flex justify-between items-center pb-2 border-b border-gray-200">
                            <span class="text-sm text-gray-600">{ "Taxes & Insurance" }</span>
                            <span class="text-lg font-semibold text-red-600">
                                { format!("-{:.0} Kč", total_tax) }
                            </span>
                        </div>
                        <div class="flex justify-between items-center pb-2 border-b border-gray-200">
                            <span class="text-sm text-gray-600">{ "Net Income (After Tax)" }</span>
                            <span class="text-lg font-semibold text-green-600">
                                { format!("{:.0} Kč", net_income) }
                            </span>
                        </div>
                        <div class="flex justify-between items-center pb-2 border-b border-gray-200">
                            <span class="text-sm text-gray-600">{ "Total Expenses" }</span>
                            <span class="text-lg font-semibold text-red-600">
                                { format!("-{:.0} Kč", total_expenses) }
                            </span>
                        </div>
                        <div class={format!(
                            "flex justify-between items-center p-3 rounded-lg {}",
                            if net_cash_flow >= Decimal::ZERO {
                                "bg-green-50"
                            } else {
                                "bg-red-50"
                            }
                        )}>
                            <span class="text-sm font-semibold text-gray-800">{ "Net Cash Flow" }</span>
                            <span class={format!(
                                "text-2xl font-bold {}",
                                if net_cash_flow >= Decimal::ZERO {
                                    "text-green-600"
                                } else {
                                    "text-red-600"
                                }
                            )}>
                                { if net_cash_flow >= Decimal::ZERO {
                                    format!("+{:.0} Kč", net_cash_flow)
                                } else {
                                    format!("{:.0} Kč", net_cash_flow)
                                }}
                            </span>
                        </div>
                    </div>
                </div>

                // Financial Summary
                <div class="bg-white rounded-lg shadow-md p-6 mb-6">
                    <h3 class="text-lg font-semibold text-gray-800 mb-4">{ "Financial Summary" }</h3>
                    <div class="grid grid-cols-2 gap-4">
                        <div class="bg-blue-50 rounded-lg p-4">
                            <p class="text-sm text-gray-600 mb-1">{ "Net Worth" }</p>
                            <p class="text-2xl font-bold text-blue-600">
                                { format!("{:.2}", net_worth) }
                                { " Kč" }
                            </p>
                            <p class="text-xs text-gray-500 mt-1">
                                // TODO: Show change from last month
                                { "—" }
                            </p>
                        </div>
                        <div class="bg-green-50 rounded-lg p-4">
                            <p class="text-sm text-gray-600 mb-1">{ "Cash Balance" }</p>
                            <p class="text-2xl font-bold text-green-600">
                                { format!("{:.2}", finances.cash) }
                                { " Kč" }
                            </p>
                            <p class="text-xs text-gray-500 mt-1">
                                { "Available for spending" }
                            </p>
                        </div>
                    </div>
                </div>

                // Player Well-being
                <div class="bg-white rounded-lg shadow-md p-6 mb-6">
                    <h3 class="text-lg font-semibold text-gray-800 mb-4">{ "Your Well-being" }</h3>
                    <div class="space-y-4">
                        <div>
                            <div class="flex justify-between mb-2">
                                <span class="text-sm text-gray-600">{ "Happiness" }</span>
                                <span class="text-sm font-semibold text-gray-800">
                                    { player.happiness }
                                    { "/100" }
                                </span>
                            </div>
                            <div class="bg-gray-200 rounded-full h-3">
                                <div
                                    class={format!("h-3 rounded-full transition-all {}",
                                        if player.happiness >= 70 { "bg-green-500" }
                                        else if player.happiness >= 40 { "bg-yellow-500" }
                                        else { "bg-red-500" }
                                    )}
                                    style={format!("width: {}%", player.happiness)}
                                ></div>
                            </div>
                        </div>

                        <div>
                            <div class="flex justify-between mb-2">
                                <span class="text-sm text-gray-600">{ "Burnout" }</span>
                                <span class="text-sm font-semibold text-gray-800">
                                    { player.burnout }
                                    { "/100" }
                                </span>
                            </div>
                            <div class="bg-gray-200 rounded-full h-3">
                                <div
                                    class={format!("h-3 rounded-full transition-all {}",
                                        if player.burnout < 30 { "bg-green-500" }
                                        else if player.burnout < 60 { "bg-yellow-500" }
                                        else { "bg-red-500" }
                                    )}
                                    style={format!("width: {}%", player.burnout)}
                                ></div>
                            </div>
                        </div>

                        <div>
                            <div class="flex justify-between mb-2">
                                <span class="text-sm text-gray-600">{ "Financial Peace Score" }</span>
                                <span class="text-sm font-semibold text-indigo-600">
                                    { financial_peace }
                                    { "/100" }
                                </span>
                            </div>
                            <div class="bg-gray-200 rounded-full h-3">
                                <div
                                    class="bg-indigo-500 h-3 rounded-full transition-all"
                                    style={format!("width: {}%", financial_peace)}
                                ></div>
                            </div>
                        </div>
                    </div>
                </div>

                // Achievements/Events (placeholder)
                <div class="bg-white rounded-lg shadow-md p-6 mb-6">
                    <h3 class="text-lg font-semibold text-gray-800 mb-4">{ "Month Highlights" }</h3>
                    <div class="space-y-3">
                        <div class="flex items-start gap-3 p-3 bg-blue-50 rounded-lg">
                            <span class="text-xl">{ "✅" }</span>
                            <div>
                                <p class="text-sm font-semibold text-gray-800">{ "Month Completed" }</p>
                                <p class="text-xs text-gray-600">
                                    { "Successfully managed your finances for 30 days" }
                                </p>
                            </div>
                        </div>

                        {if finances.has_emergency_fund() {
                            html! {
                                <div class="flex items-start gap-3 p-3 bg-green-50 rounded-lg">
                                    <span class="text-xl">{ "🛡️" }</span>
                                    <div>
                                        <p class="text-sm font-semibold text-gray-800">{ "Emergency Fund Complete" }</p>
                                        <p class="text-xs text-gray-600">
                                            { "You have 3 months of expenses saved!" }
                                        </p>
                                    </div>
                                </div>
                            }
                        } else {
                            html! {}
                        }}

                        {if player.is_revenge_spending_risk() {
                            html! {
                                <div class="flex items-start gap-3 p-3 bg-yellow-50 rounded-lg">
                                    <span class="text-xl">{ "⚠️" }</span>
                                    <div>
                                        <p class="text-sm font-semibold text-gray-800">{ "Watch Your Well-being" }</p>
                                        <p class="text-xs text-gray-600">
                                            { "Low happiness or high burnout may lead to impulse spending" }
                                        </p>
                                    </div>
                                </div>
                            }
                        } else {
                            html! {}
                        }}
                    </div>
                </div>

                // Action Button
                <div class="flex justify-center">
                    <button
                        onclick={on_continue}
                        class="bg-gradient-to-r from-green-500 to-emerald-600 text-white font-bold py-4 px-8 rounded-lg hover:from-green-600 hover:to-emerald-700 transform transition hover:scale-105 shadow-lg text-lg"
                    >
                        { "Continue to Next Month →" }
                    </button>
                </div>
            </div>
        </div>
    }
}
