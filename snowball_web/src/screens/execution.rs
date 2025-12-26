use fin_engine::{GamePhase, GameState};
use gloo_timers::callback::Interval;
use yew::prelude::*;

#[derive(Clone, Copy, PartialEq)]
enum PlaybackSpeed {
    Slow,   // 2000ms
    Normal, // 1000ms
    Fast,   // 500ms
}

impl PlaybackSpeed {
    fn to_millis(self) -> u32 {
        match self {
            PlaybackSpeed::Slow => 2000,
            PlaybackSpeed::Normal => 1000,
            PlaybackSpeed::Fast => 500,
        }
    }

    fn label(self) -> &'static str {
        match self {
            PlaybackSpeed::Slow => "1x",
            PlaybackSpeed::Normal => "2x",
            PlaybackSpeed::Fast => "4x",
        }
    }

    fn next(self) -> Self {
        match self {
            PlaybackSpeed::Slow => PlaybackSpeed::Normal,
            PlaybackSpeed::Normal => PlaybackSpeed::Fast,
            PlaybackSpeed::Fast => PlaybackSpeed::Slow,
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct ExecutionProps {
    pub game_state: GameState,
    pub on_advance_day: Callback<()>,
}

#[function_component(ExecutionScreen)]
pub fn execution_screen(props: &ExecutionProps) -> Html {
    let game_state = &props.game_state;
    let player = &game_state.player;
    let finances = &game_state.finances;

    let current_day = if let GamePhase::Execution { current_day } = game_state.phase {
        current_day
    } else {
        1
    };

    let is_playing = use_state(|| true); // Start playing by default
    let is_skipping = use_state(|| false); // Track if we're skipping to end
    let speed = use_state(|| PlaybackSpeed::Normal);
    let progress_percent = (current_day as f32 / 30.0 * 100.0) as u8;

    // Auto-advance timer
    {
        let on_advance_day = props.on_advance_day.clone();
        let is_playing = is_playing.clone();
        let is_skipping = is_skipping.clone();
        let speed = *speed;
        let current_day = current_day;

        use_effect_with(
            (current_day, *is_playing, *is_skipping, speed),
            move |(_, playing, skipping, speed)| {
                let interval = if (*playing || *skipping) && current_day < 30 {
                    // Use very fast interval (50ms) when skipping, normal speed otherwise
                    let interval_ms = if *skipping { 50 } else { speed.to_millis() };

                    // TODO: In the future, check for events here
                    // If an event occurs during skipping, pause the skip by setting is_skipping to false
                    // Example: if has_event_on_day(current_day) && *skipping { is_skipping.set(false); }

                    Some(Interval::new(interval_ms, move || {
                        on_advance_day.emit(());
                    }))
                } else {
                    None
                };

                // Return cleanup function that drops interval if it exists
                move || drop(interval)
            },
        );
    }

    let on_toggle_play = {
        let is_playing = is_playing.clone();
        let is_skipping = is_skipping.clone();
        Callback::from(move |_| {
            // Stop skipping if we're toggling play
            if *is_skipping {
                is_skipping.set(false);
            }
            is_playing.set(!*is_playing);
        })
    };

    let on_change_speed = {
        let speed = speed.clone();
        Callback::from(move |_| {
            speed.set((*speed).next());
        })
    };

    let on_skip_to_end = {
        let is_skipping = is_skipping.clone();
        let is_playing = is_playing.clone();
        Callback::from(move |_| {
            // Pause normal playback and start skipping
            is_playing.set(false);
            is_skipping.set(true);
        })
    };

    let on_next_day = {
        let on_advance_day = props.on_advance_day.clone();
        Callback::from(move |_| {
            on_advance_day.emit(());
        })
    };

    html! {
        <div class="min-h-screen bg-gradient-to-br from-purple-50 to-pink-100">
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
                                { " - Day " }
                                { current_day }
                            </p>
                        </div>
                        <div class="text-right">
                            <p class="text-xs text-gray-500">{ "Cash Balance" }</p>
                            <p class="text-lg font-bold text-gray-800">
                                { format!("{:.2}", finances.cash) }
                                { " Kč" }
                            </p>
                        </div>
                    </div>
                </div>
            </div>

            // Main Content
            <div class="max-w-4xl mx-auto px-4 py-8">
                // Phase Indicator
                <div class="bg-purple-500 text-white rounded-lg p-6 mb-6 shadow-lg">
                    <div class="flex items-center justify-between mb-4">
                        <div>
                            <h2 class="text-2xl font-bold mb-1">{ "Month Execution" }</h2>
                            <p class="text-purple-100">
                                { "Phase 2: Watch the month unfold day by day" }
                            </p>
                        </div>
                        <div class="bg-purple-400 rounded-full w-16 h-16 flex items-center justify-center">
                            <span class="text-3xl">{ "📅" }</span>
                        </div>
                    </div>

                    // Progress Bar
                    <div class="bg-purple-400 rounded-full h-4 overflow-hidden">
                        <div
                            class="bg-white h-4 transition-all duration-300 flex items-center justify-center text-xs font-bold text-purple-600"
                            style={format!("width: {}%", progress_percent)}
                        >
                            {if progress_percent > 15 {
                                html! { { format!("Day {}/30", current_day) } }
                            } else {
                                html! {}
                            }}
                        </div>
                    </div>
                </div>

                // Current Day Display
                <div class="bg-white rounded-lg shadow-md p-8 mb-6 text-center">
                    <div class="mb-4">
                        <span class="text-6xl">{ "📆" }</span>
                    </div>
                    <h3 class="text-3xl font-bold text-gray-800 mb-2">
                        { "Day " }
                        { current_day }
                    </h3>
                    <p class="text-gray-600 mb-6">
                        { game_state.time.month.name() }
                        { " " }
                        { game_state.time.day }
                        { ", " }
                        { game_state.time.year }
                    </p>

                    // Daily Status
                    <div class="bg-gray-50 rounded-lg p-4 mb-6">
                        <p class="text-sm text-gray-600 mb-3">{ "Daily Status" }</p>
                        <div class="flex justify-around text-center">
                            <div>
                                <p class="text-xs text-gray-500 mb-1">{ "Happiness" }</p>
                                <p class="text-lg font-bold text-gray-800">{ player.happiness }</p>
                            </div>
                            <div>
                                <p class="text-xs text-gray-500 mb-1">{ "Burnout" }</p>
                                <p class="text-lg font-bold text-gray-800">{ player.burnout }</p>
                            </div>
                            <div>
                                <p class="text-xs text-gray-500 mb-1">{ "Peace Score" }</p>
                                <p class="text-lg font-bold text-indigo-600">
                                    { player.financial_peace_score() }
                                </p>
                            </div>
                        </div>
                    </div>
                </div>

                // Events/Activities (placeholder)
                <div class="bg-white rounded-lg shadow-md p-6 mb-6">
                    <h3 class="text-lg font-semibold text-gray-800 mb-4">{ "Today's Activities" }</h3>
                    <div class="space-y-3">
                        <div class="flex items-center gap-3 p-3 bg-blue-50 rounded-lg">
                            <span class="text-2xl">{ "💼" }</span>
                            <div>
                                <p class="text-sm font-semibold text-gray-800">{ "Regular Day" }</p>
                                <p class="text-xs text-gray-600">
                                    { "No special events today. Time passes..." }
                                </p>
                            </div>
                        </div>

                        {if current_day % 7 == 0 {
                            html! {
                                <div class="flex items-center gap-3 p-3 bg-green-50 rounded-lg">
                                    <span class="text-2xl">{ "🎉" }</span>
                                    <div>
                                        <p class="text-sm font-semibold text-gray-800">{ "Weekend!" }</p>
                                        <p class="text-xs text-gray-600">
                                            { "Time to relax and recharge. Happiness +5" }
                                        </p>
                                    </div>
                                </div>
                            }
                        } else {
                            html! {}
                        }}
                    </div>
                </div>

                // Playback Controls
                <div class="flex flex-col items-center gap-4">
                    {if current_day < 30 {
                        html! {
                            <>
                                <div class="bg-white rounded-lg shadow-md p-4 flex items-center gap-4 flex-wrap justify-center">
                                    {if !*is_skipping {
                                        html! {
                                            <>
                                                // Play/Pause Button
                                                <button
                                                    onclick={on_toggle_play}
                                                    class={format!(
                                                        "font-bold py-3 px-6 rounded-lg transition transform hover:scale-105 shadow {}",
                                                        if *is_playing {
                                                            "bg-yellow-500 hover:bg-yellow-600 text-white"
                                                        } else {
                                                            "bg-green-500 hover:bg-green-600 text-white"
                                                        }
                                                    )}
                                                >
                                                    {if *is_playing {
                                                        html! { <>{ "⏸ Pause" }</> }
                                                    } else {
                                                        html! { <>{ "▶ Play" }</> }
                                                    }}
                                                </button>

                                                // Speed Control
                                                <button
                                                    onclick={on_change_speed}
                                                    class="bg-indigo-500 hover:bg-indigo-600 text-white font-bold py-3 px-6 rounded-lg transition transform hover:scale-105 shadow"
                                                >
                                                    { "Speed: " }
                                                    { (*speed).label() }
                                                </button>

                                                // Skip to End Button
                                                <button
                                                    onclick={on_skip_to_end}
                                                    class="bg-orange-500 hover:bg-orange-600 text-white font-bold py-3 px-6 rounded-lg transition transform hover:scale-105 shadow"
                                                >
                                                    { "⏩ Skip to End" }
                                                </button>

                                                // Manual Advance (when paused)
                                                {if !*is_playing {
                                                    html! {
                                                        <button
                                                            onclick={on_next_day}
                                                            class="bg-purple-500 hover:bg-purple-600 text-white font-bold py-3 px-6 rounded-lg transition transform hover:scale-105 shadow"
                                                        >
                                                            { "Next Day →" }
                                                        </button>
                                                    }
                                                } else {
                                                    html! {}
                                                }}
                                            </>
                                        }
                                    } else {
                                        html! {
                                            <div class="flex items-center gap-3 py-2">
                                                <span class="text-2xl animate-pulse">{ "⏩" }</span>
                                                <span class="text-lg font-semibold text-gray-800">
                                                    { "Skipping to end of month..." }
                                                </span>
                                            </div>
                                        }
                                    }}
                                </div>

                                <p class="text-sm text-gray-600 text-center">
                                    {if *is_skipping {
                                        "Rapidly advancing through remaining days. Will stop if events occur."
                                    } else if *is_playing {
                                        "Days are advancing automatically. Click pause to stop or skip to end."
                                    } else {
                                        "Click play to auto-advance, skip to end, or manually step through days."
                                    }}
                                </p>
                            </>
                        }
                    } else {
                        html! {
                            <div class="text-center">
                                <div class="bg-green-50 border-2 border-green-500 rounded-lg p-6 mb-4">
                                    <p class="text-green-800 font-semibold mb-2">
                                        { "Month Complete!" }
                                    </p>
                                    <p class="text-sm text-green-700">
                                        { "Ready to review your progress" }
                                    </p>
                                </div>
                                <button
                                    onclick={on_next_day}
                                    class="bg-gradient-to-r from-green-500 to-emerald-600 text-white font-bold py-4 px-8 rounded-lg hover:from-green-600 hover:to-emerald-700 transform transition hover:scale-105 shadow-lg text-lg"
                                >
                                    { "View Review →" }
                                </button>
                            </div>
                        }
                    }}
                </div>
            </div>
        </div>
    }
}
