use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>


        // sets the document title
        <Title text="DragonCodes"/>

        // content for this welcome page
        <Router>
            <main class="flex min-h-screen flex-col items-center justify-center bg-[#000]">
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {

       <AboutMe />

       <Socials />
    }
}

#[component]
fn AboutMe() -> impl IntoView {
    view! {
        <div class="bg-[#f2fff8] rounded-xl mt-10">
            <div class="max-w-7xl mx-auto py-16 px-4 sm:px-6 lg:px-8">
                <div class="text-center">
                    <h2 class="text-base font-semibold text-green-600 tracking-wide uppercase">Hello, friend!</h2>
                    <p class="mt-1 text-4xl font-extrabold text-gray-900 sm:text-5xl sm:tracking-tight lg:text-6xl">Welcome to my site <div class="text-sm text-slate-400">(written in Rust by the way)</div></p>
                    <p class="max-w-xl mt-5 mx-auto text-xl text-gray-500">I am a developer, I build stuff</p>
                    <p class="max-w-xl mt-5 mx-auto text-xl text-gray-500">When I am bored I <a target="_blank" class="text-sky-400" href="https://musings-theta.vercel.app/">amuse</a> myself</p>
                </div>
            </div>
        </div>
    }
}

#[component]
fn Socials() -> impl IntoView {
    view! {
        <div class="flex-1 flex mt-10">
            <a href="https://github.com/dragoncodes" target="_blank" class="w-[50px]">
                <img class="rounded-xl" src="https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png" alt="GitHub" />
            </a>

            <a href="https://medium.com/@dragoncodes" target="_blank" class="flex justify-center h-[50px] ml-3 border border-slate-400 rounded-xl">
               <img class="w-[50px]"  src="/assets/medium.png" alt="Medium" />
            </a>
        </div>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
