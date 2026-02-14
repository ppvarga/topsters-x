use leptos::{html::Canvas, prelude::*, svg::Image, wasm_bindgen::JsCast};
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
use web_sys::{CanvasRenderingContext2d, HtmlImageElement, wasm_bindgen::prelude::Closure};
use crate::chart::{self, Album, Chart, ChartPreset};

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

fn example_chart_9() -> RwSignal<Chart> {
    let mut chart = Chart::from_preset(ChartPreset::Nine);
    chart.rows[0][0] = Some(Album { id: 1.to_string(), title: "".to_string(), artist: "".to_string(), cover_url: "https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fis1-ssl.mzstatic.com%2Fimage%2Fthumb%2FMusic126%2Fv4%2F45%2F55%2F22%2F455522bc-9610-e655-548e-c246295cd1a6%2F744861133063.png%2F1200x1200bf-60.jpg&f=1&nofb=1&ipt=0245568aca710daedc86f42fbbc44bde1f8e3f209b14d7d5ce207fbf480c2510".to_string() });
    chart.rows[0][1] = Some(Album { id: 1.to_string(), title: "".to_string(), artist: "".to_string(), cover_url: "https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fwww.offtherecord.com.gr%2Fwp-content%2Fuploads%2F2011%2F11%2Fking-crimson-court.jpg&f=1&nofb=1&ipt=3dd00984ddf82310ff9b3b15a555710dfc306d95bdb2260f1ea67b7a64c81177".to_string() });
    chart.rows[0][2] = Some(Album { id: 1.to_string(), title: "".to_string(), artist: "".to_string(), cover_url: "https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fcdn.albumoftheyear.org%2Falbum%2F363-in-rainbows_090114.jpg&f=1&nofb=1&ipt=595180cbfb1b4c0e02a7e556870e598a3a21b6c0cd6463caa491edcfa5654777".to_string() });
    chart.rows[1][0] = Some(Album { id: 1.to_string(), title: "".to_string(), artist: "".to_string(), cover_url: "https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fwww.mrvinyl.co.za%2Fwp-content%2Fuploads%2F2018%2F10%2Fsoad-toxicity.jpg&f=1&nofb=1&ipt=feab274fe03489f3dadc2e84fdd406d919f8ca3395629463487f37544beaba49".to_string() });
    chart.rows[1][1] = Some(Album { id: 1.to_string(), title: "".to_string(), artist: "".to_string(), cover_url: "https://external-content.duckduckgo.com/iu/?u=http%3A%2F%2Fbandwagon-gig-finder.s3.amazonaws.com%2Feditorials%2Fuploads%2Fpictures%2F1344%2Fcontent_Rage-Against-the-Machine-self-titled-cover.jpg&f=1&nofb=1&ipt=f46c89ab75988503865bb0ef5ece56881a200ee848f6a9aa71a1f176530df1ba".to_string() });
    chart.rows[1][2] = None; // Some(Album { id: 1.to_string(), title: "".to_string(), artist: "".to_string(), cover_url: "https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fimages.genius.com%2F1267aea8f8e715255a06f9b6bb4026b9.1000x1000x1.png&f=1&nofb=1&ipt=68aac42a10daba3f3636bb41f0f0fbc1cecfd658376f0e43c5ce58f173984fb1".to_string() });
    chart.rows[2][0] = Some(Album { id: 1.to_string(), title: "".to_string(), artist: "".to_string(), cover_url: "https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fpinkfloydhyperbase.dk%2Fillu%2Fcovers%2Fanimals.jpg&f=1&nofb=1&ipt=f02d3ca7893bf73516d2115352df5741c6274434248ebddfe11fbabf13344c7d".to_string() });
    chart.rows[2][1] = Some(Album { id: 1.to_string(), title: "".to_string(), artist: "".to_string(), cover_url: "https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fsuperdeluxeedition.com%2Fwp-content%2Fuploads%2F2023%2F01%2Fdeadwing_front.jpg&f=1&nofb=1&ipt=a211e820ba24225db6ce51346cb669bead1ce04392bd56c3a6f1c10455cafc67".to_string() });
    chart.rows[2][2] = Some(Album { id: 1.to_string(), title: "".to_string(), artist: "".to_string(), cover_url: "https://external-content.duckduckgo.com/iu/?u=http%3A%2F%2Fimages.genius.com%2F2b44f37ccfab7ed0c431af36a0bda79c.1000x1000x1.jpg&f=1&nofb=1&ipt=32961e97598e5bea9abc9bd1a18ab493341ab6706ba7e527ec4ed922eed26ad9".to_string() });
    RwSignal::new(chart)

}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let canvas = NodeRef::<Canvas>::new();
    let chart = example_chart_9();

    const OUTER_MARGIN: f64 = 20.0;
    const INNER_MARGIN: f64 = 10.0;
    const CANVAS_WIDTH: f64 = 500.0;
    const CANVAS_HEIGHT: f64 = 1000.0;

    Effect::new(move |_| {
        let chart = chart.get();

        let Some(canvas_el) = canvas.get() else { return };

        let ctx= canvas_el
            .get_context("2d")
            .ok()
            .flatten()
            .and_then(|v| v.dyn_into::<CanvasRenderingContext2d>().ok());

        let Some(ctx) = ctx else { return };
        
        let mut dx: f64 = OUTER_MARGIN;
        let mut dy: f64 = OUTER_MARGIN;
        
        for (i, row_length) in chart.row_sizes.iter().enumerate() {
            let n: f64 = *row_length as f64;
            let width: f64 = (CANVAS_WIDTH - 2.0*OUTER_MARGIN - (n-1.0)*INNER_MARGIN) / n;

            for j in 0..*row_length {
                match &chart.rows[i][j] {
                    None => {
                        ctx.set_fill_style_str("#333333");
                        ctx.fill_rect(dx, dy, width, width);
                    }
                    Some(Album{id, title, artist, cover_url}) => {
                        draw_image_from_url(ctx.clone(), cover_url.to_string(), dx, dy, width, width);
                    }
                }

                dx = dx + INNER_MARGIN + width;
            }

            dx = OUTER_MARGIN;
            dy = dy + INNER_MARGIN + width;
        }

        // Clear
        // ctx.clear_rect(0.0, 0.0, canvas_el.width() as f64, canvas_el.height() as f64);

        // Draw something based on `c`
        // ctx.set_font("20px sans-serif");
        // ctx.fill_text(&format!("count = {c}"), 20.0, 40.0).ok();

        // Example: draw a grid that grows with count
        // let step = (20 + (c % 20)) as f64;
        // let w = canvas_el.width() as f64;
        // let h = canvas_el.height() as f64;

        // for x in (0..).map(|i| i as f64 * step).take_while(|&x| x <= w) {
        //     ctx.begin_path();
        //     ctx.move_to(x, 0.0);
        //     ctx.line_to(x, h);
        //     ctx.stroke();
        // }
        // for y in (0..).map(|i| i as f64 * step).take_while(|&y| y <= h) {
        //     ctx.begin_path();
        //     ctx.move_to(0.0, y);
        //     ctx.line_to(w, y);
        //     ctx.stroke();
        // }
    });

    // let on_click = move |_| {
    //     *count.write() += 1;
    // };

    
    view! {
        <h1>"Kishaver, te mán nyomod!"</h1>
        // <button on:click=on_click>"Click Meeee PLEASE: " {count}</button>
        <canvas id="grid" width="500" height="500" node_ref=canvas></canvas>
    }
}

fn draw_image_from_url(
    ctx: CanvasRenderingContext2d,
    url: String,
    x: f64,
    y: f64,
    w: f64,
    h: f64,
) {
    let img = HtmlImageElement::new().unwrap();

    // If the image host sends CORS headers and you want to export the canvas later:
    // img.set_cross_origin(Some("anonymous"));

    let img_clone = img.clone();
    let onload = Closure::<dyn FnMut()>::new(move || {
        // drawImage(img, x, y, w, h)
        let _ = ctx.draw_image_with_html_image_element_and_dw_and_dh(&img_clone, x, y, w, h);
    });

    img.set_onload(Some(onload.as_ref().unchecked_ref()));
    img.set_src(&url);

    // Important: keep the closure alive until it runs
    onload.forget();
}