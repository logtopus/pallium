extern crate cfg_if;
extern crate wasm_bindgen;

use cfg_if::cfg_if;
// use futures::{future, Future};
// use js_sys::Promise;
// use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
// use wasm_bindgen_futures::future_to_promise;
// use wasm_bindgen_futures::JsFuture;
use web_sys::console;
use web_sys::{HtmlElement, Request, RequestInit, RequestMode, Response};

mod utils;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

fn update_source_choice(document: &web_sys::Document, pallium: &Pallium) {
    let src_choice = document
        .get_element_by_id("SourceChoice")
        .expect("document should have source choice");
    pallium.log_sources.iter().for_each(|src| {
        let opt = document
            .create_element("option")
            .expect("Failed to create source option");
        opt.set_inner_html(src);
        src_choice
            .append_child(&opt)
            .expect("Failed to add source option");
    });
}

fn register_refresh_handler(document: &web_sys::Document, pallium: Box<Pallium>) {
    let refreshlog_handler = Closure::wrap(Box::new(move || {
        pallium.request_log("system-auth");
        // num_clicks.set_inner_html(&clicks.to_string());
    }) as Box<dyn FnMut()>);

    document
        .get_element_by_id("refreshlog")
        .expect("should have refreshlog item on the page")
        .dyn_ref::<HtmlElement>()
        .expect("refreshlog item should be a `HtmlElement`")
        .set_onclick(Some(refreshlog_handler.as_ref().unchecked_ref()));
    refreshlog_handler.forget();
}

#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let origin = window.location().origin()?;

    let document = window.document().expect("should have a document on window");
    // let body = document.body().expect("document should have a body");

    let pallium = Pallium::new(origin);

    update_source_choice(&document, &pallium);
    register_refresh_handler(&document, Box::new(pallium));

    let content = document
        .get_element_by_id("Content")
        .expect("document should have conent region");

    for i in 0..200 {
        let tr = document
            .create_element("tr")
            .expect("Failed to create table row");

        let td = document
            .create_element("td")
            .expect("Failed to create table column");
        td.set_text_content(Some("2019-03-10 15:42:09"));
        tr.append_child(&td).expect("Failed to add table column");

        let td = document
            .create_element("td")
            .expect("Failed to create table column");
        if i % 3 == 0 {
            td.set_class_name("error");
            td.set_text_content(Some("ERROR"));
        } else if i % 3 == 1 {
            td.set_class_name("warning");
            td.set_text_content(Some("WARN"));
        } else {
            td.set_class_name("debug");
            td.set_text_content(Some("DEBUG"));
        }
        tr.append_child(&td).expect("Failed to add table column");

        let td = document
            .create_element("td")
            .expect("Failed to create table column");
        td.set_text_content(Some(&format!("This is a warning message {}", i)));
        tr.append_child(&td).expect("Failed to add table column");

        content.append_child(&tr).expect("Failed to add table row");
    }

    Ok(())
}

struct Pallium {
    pub origin: String,
    pub log_sources: Vec<String>,
}

impl Pallium {
    pub fn new(origin: String) -> Self {
        Pallium {
            origin,
            log_sources: vec!["system-auth".to_owned(), "system-syslog".to_owned()],
        }
    }

    pub fn request_log(&self, source: &str) {
        let mut opts = RequestInit::new();
        opts.method("GET");
        opts.mode(RequestMode::Cors);

        let url = format!(
            "{}/api/v1/sources/{}/content?from_ms=0",
            &self.origin, &source
        );
        console::log_1(&url.clone().into());
        let request = Request::new_with_str_and_init(&url, &opts).unwrap();
    }
}
