use leptos::{html::Canvas, prelude::*,wasm_bindgen::JsCast};
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
use web_sys::{CanvasRenderingContext2d};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <link rel="preconnect" href="https://fonts.googleapis.com"/>
                <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin/>
                <link href="https://fonts.googleapis.com/css2?family=Montserrat:ital,wght@0,100..900;1,100..900&display=swap" rel="stylesheet"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/topsters-x.css"/>

        // sets the document title
        <Title text="Topsters X"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let canvas = NodeRef::<Canvas>::new();
    let count = RwSignal::new(0);

    Effect::new(move |_| {
        let c = count.get(); 

        let Some(canvas_el) = canvas.get() else { return };

        let ctx= canvas_el
            .get_context("2d")
            .ok()
            .flatten()
            .and_then(|v| v.dyn_into::<CanvasRenderingContext2d>().ok());

        let Some(ctx) = ctx else { return };

        // Clear
        ctx.clear_rect(0.0, 0.0, canvas_el.width() as f64, canvas_el.height() as f64);

        // Draw something based on `c`
        ctx.set_font("20px sans-serif");
        ctx.fill_text(&format!("count = {c}"), 20.0, 40.0).ok();

        // Example: draw a grid that grows with count
        let step = (20 + (c % 20)) as f64;
        let w = canvas_el.width() as f64;
        let h = canvas_el.height() as f64;

        for x in (0..).map(|i| i as f64 * step).take_while(|&x| x <= w) {
            ctx.begin_path();
            ctx.move_to(x, 0.0);
            ctx.line_to(x, h);
            ctx.stroke();
        }
        for y in (0..).map(|i| i as f64 * step).take_while(|&y| y <= h) {
            ctx.begin_path();
            ctx.move_to(0.0, y);
            ctx.line_to(w, y);
            ctx.stroke();
        }
    });

    let on_click = move |_| {
        *count.write() += 1;
        // change something about the canvas
    };

    
    view! {
        <h1>"Kishaver, te mán nyomod!"</h1>
        <button on:click=on_click>"Click Meeee PLEASE: " {count}</button>
        <canvas id="grid" width="500" height="500" node_ref=canvas></canvas>
    }
}
