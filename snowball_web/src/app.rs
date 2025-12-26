use crate::app_state::{AppAction, AppState};
use crate::components::{Initialization, InitializationData};
use crate::screens::{ExecutionScreen, PlanningScreen, ReviewScreen};
use fin_engine::{CzechMarket, GamePhase, GameState};
use yew::prelude::*;

/// Gets the market profile for a given market ID
fn get_market_profile(market_id: &str) -> Box<dyn fin_engine::market::MarketProfile> {
    match market_id {
        "czech" => Box::new(CzechMarket),
        _ => Box::new(CzechMarket), // Default to Czech for now
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let app_state = use_reducer(AppState::default);

    let on_start_game = {
        let app_state = app_state.clone();
        Callback::from(move |data: InitializationData| {
            // Create a unique save ID
            let save_id = format!("save_{}", js_sys::Date::now() as u64);

            // Get current year
            let current_year = js_sys::Date::new_0().get_full_year();

            // Create new game state
            match GameState::new(
                save_id,
                data.market_id.clone(),
                data.player_name,
                data.player_age,
                current_year,
            ) {
                Ok(mut game_state) => {
                    // If a starting job was selected, accept it and setup initial finances
                    if let Some(job) = data.starting_job {
                        // Give starting cash (50% of monthly salary)
                        game_state.finances.cash =
                            job.monthly_salary / rust_decimal::Decimal::from(2);

                        // Set minimum food budget (3,500 Kč/month - survival level)
                        game_state.finances.set_budget(
                            fin_engine::ExpenseCategory::Essential,
                            rust_decimal::Decimal::from(3500),
                        );

                        // Accept the job
                        game_state.career.accept_job(job.clone());

                        // Create income entry for the job
                        let income_id = format!("job_{}", job.id);
                        game_state.finances.income_sources.push(fin_engine::Income {
                            id: income_id,
                            name: job.title.clone(),
                            kind: fin_engine::IncomeKind::Employment,
                            gross_monthly: job.monthly_salary,
                            active: true,
                        });
                    }

                    app_state.dispatch(AppAction::StartGame(game_state));
                }
                Err(e) => {
                    web_sys::console::error_1(
                        &format!("Failed to create game state: {}", e).into(),
                    );
                }
            }
        })
    };

    let on_start_month = {
        let app_state = app_state.clone();
        Callback::from(move |_| {
            if let AppState::Playing { game_state } = &*app_state {
                let mut new_state = (**game_state).clone();
                new_state.advance_phase();
                app_state.dispatch(AppAction::UpdateGameState(new_state));
            }
        })
    };

    let on_advance_day = {
        let app_state = app_state.clone();
        Callback::from(move |_| {
            if let AppState::Playing { game_state } = &*app_state {
                let mut new_state = (**game_state).clone();
                let market = get_market_profile(&new_state.market_id);
                match new_state.advance_execution_day(market.as_ref()) {
                    Ok(_) => {
                        // If we've reached day 30, automatically advance to review
                        if matches!(new_state.phase, GamePhase::Review) {
                            // Phase already transitioned by advance_execution_day
                            // Financial settlement has been processed
                        }
                        app_state.dispatch(AppAction::UpdateGameState(new_state));
                    }
                    Err(e) => {
                        web_sys::console::error_1(&format!("Error advancing day: {}", e).into());
                    }
                }
            }
        })
    };

    let on_next_month = {
        let app_state = app_state.clone();
        Callback::from(move |_| {
            if let AppState::Playing { game_state } = &*app_state {
                let mut new_state = (**game_state).clone();
                new_state.advance_phase(); // Review -> Planning, advances month
                app_state.dispatch(AppAction::UpdateGameState(new_state));
            }
        })
    };

    let on_update_state = {
        let app_state = app_state.clone();
        Callback::from(move |new_state: GameState| {
            app_state.dispatch(AppAction::UpdateGameState(new_state));
        })
    };

    match &*app_state {
        AppState::Initialization => {
            html! {
                <Initialization on_start={on_start_game} />
            }
        }
        AppState::Playing { game_state } => match game_state.phase {
            GamePhase::Planning => {
                html! {
                    <PlanningScreen
                        game_state={(**game_state).clone()}
                        on_start_month={on_start_month}
                        on_update_state={on_update_state.clone()}
                    />
                }
            }
            GamePhase::Execution { .. } => {
                html! {
                    <ExecutionScreen
                        game_state={(**game_state).clone()}
                        on_advance_day={on_advance_day}
                    />
                }
            }
            GamePhase::Review => {
                html! {
                    <ReviewScreen
                        game_state={(**game_state).clone()}
                        on_next_month={on_next_month}
                    />
                }
            }
        },
    }
}
