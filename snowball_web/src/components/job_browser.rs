use fin_engine::{Career, Job, JobMarket};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct JobBrowserProps {
    pub career: Career,
    pub market_id: String,
    pub on_accept_job: Callback<Job>,
    pub on_close: Callback<()>,
}

#[function_component(JobBrowser)]
pub fn job_browser(props: &JobBrowserProps) -> Html {
    let career = &props.career;

    // Generate available jobs based on career
    let available_jobs = if props.market_id == "czech" {
        JobMarket::generate_czech_jobs(career)
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
                <div class="bg-gradient-to-r from-purple-500 to-indigo-600 text-white p-6">
                    <div class="flex justify-between items-center">
                        <div>
                            <h2 class="text-2xl font-bold mb-1">{ "Job Market" }</h2>
                            <p class="text-purple-100 text-sm">
                                { format!("Experience: {} years", career.years_experience) }
                                { " • Qualified for: " }
                                { career.max_qualified_level().name() }
                            </p>
                        </div>
                        <button
                            onclick={on_close_click}
                            class="text-white hover:bg-purple-600 rounded-full w-10 h-10 flex items-center justify-center transition"
                        >
                            { "✕" }
                        </button>
                    </div>
                </div>

                // Current Job Section
                <div class="p-6 border-b border-gray-200">
                    <h3 class="text-lg font-semibold text-gray-800 mb-3">{ "Current Employment" }</h3>
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
                                        <p class="text-xs text-gray-500">
                                            { format!("{} months at this position", career.months_in_current_job) }
                                        </p>
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
                            <div class="bg-gray-50 rounded-lg p-4 text-center">
                                <p class="text-gray-500">{ "Currently unemployed" }</p>
                                <p class="text-sm text-gray-400 mt-1">{ "Browse available positions below" }</p>
                            </div>
                        }
                    }}
                </div>

                // Available Jobs List
                <div class="p-6 overflow-y-auto max-h-96">
                    <h3 class="text-lg font-semibold text-gray-800 mb-4">{ "Available Positions" }</h3>

                    {if available_jobs.is_empty() {
                        html! {
                            <div class="text-center py-8">
                                <p class="text-gray-500">{ "No jobs available at your level" }</p>
                                <p class="text-sm text-gray-400 mt-2">
                                    { "Keep working to gain more experience!" }
                                </p>
                            </div>
                        }
                    } else {
                        html! {
                            <div class="space-y-3">
                                {available_jobs.iter().map(|job| {
                                    let qualifies = job.qualifies(career.years_experience);
                                    let job_clone = job.clone();
                                    let on_accept = {
                                        let on_accept_job = props.on_accept_job.clone();
                                        Callback::from(move |_| {
                                            on_accept_job.emit(job_clone.clone());
                                        })
                                    };

                                    let is_current = career.current_job.as_ref().map(|j| j.id == job.id).unwrap_or(false);

                                    html! {
                                        <div
                                            key={job.id.clone()}
                                            class={format!(
                                                "border-2 rounded-lg p-4 transition {}",
                                                if is_current {
                                                    "border-green-500 bg-green-50"
                                                } else if !qualifies {
                                                    "border-gray-200 bg-gray-50 opacity-60"
                                                } else {
                                                    "border-gray-200 hover:border-indigo-300 hover:bg-indigo-50"
                                                }
                                            )}
                                        >
                                            <div class="flex justify-between items-start mb-3">
                                                <div class="flex-1">
                                                    <div class="flex items-center gap-2 mb-1">
                                                        <p class="text-lg font-bold text-gray-800">{ &job.title }</p>
                                                        <span class={format!(
                                                            "text-xs px-2 py-1 rounded {}",
                                                            if qualifies { "bg-green-100 text-green-700" } else { "bg-red-100 text-red-700" }
                                                        )}>
                                                            { job.level_name() }
                                                        </span>
                                                    </div>
                                                    <p class="text-sm text-gray-600">
                                                        {if let Some(company) = &job.company {
                                                            html! { <>{ company }{ " • " }</> }
                                                        } else {
                                                            html! {}
                                                        }}
                                                        { job.field.name() }
                                                    </p>
                                                    <p class="text-xs text-gray-500 mt-1">
                                                        { format!("Requires {} years experience", job.required_experience) }
                                                        {if !qualifies {
                                                            html! {
                                                                <span class="text-red-600 ml-2">
                                                                    { format!("(You have {})", career.years_experience) }
                                                                </span>
                                                            }
                                                        } else {
                                                            html! {}
                                                        }}
                                                    </p>
                                                </div>
                                                <div class="text-right">
                                                    <p class="text-xl font-bold text-gray-800">
                                                        { format!("{:.0}", job.monthly_salary) }
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
                                                        { "Current Job" }
                                                    </button>
                                                }
                                            } else if !qualifies {
                                                html! {
                                                    <button
                                                        class="w-full bg-gray-300 text-gray-600 font-semibold py-2 px-4 rounded cursor-not-allowed"
                                                        disabled=true
                                                    >
                                                        { "Not Qualified" }
                                                    </button>
                                                }
                                            } else {
                                                html! {
                                                    <button
                                                        onclick={on_accept}
                                                        class="w-full bg-gradient-to-r from-indigo-500 to-purple-600 text-white font-semibold py-2 px-4 rounded hover:from-indigo-600 hover:to-purple-700 transition transform hover:scale-105"
                                                    >
                                                        {if career.is_employed() {
                                                            "Switch to This Job"
                                                        } else {
                                                            "Accept Job Offer"
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
                        { "Tip: Gain experience by working to unlock higher-paying positions" }
                    </p>
                </div>
            </div>
        </div>
    }
}
