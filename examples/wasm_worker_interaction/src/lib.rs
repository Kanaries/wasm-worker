use quad_rand as qrand;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, HtmlElement, HtmlInputElement, MessageEvent, Worker};

/// A number evaluation struct
///
/// This struct will be the main object which responds to messages passed to the worker. It stores
/// the last number which it was passed to have a state. The statefulness is not is not required in
/// this example but should show how larger, more complex scenarios with statefulness can be set up.
#[wasm_bindgen]
pub struct NumberEval {
    number: i32,
}

#[wasm_bindgen]
impl NumberEval {
    /// Create new instance.
    pub fn new(init_number: i32) -> NumberEval {
        NumberEval {
            number: init_number,
        }
    }

    /// Get last number that was checked - this method is added to work with statefulness.
    pub fn get_last_number(&self) -> i32 {
        self.number
    }

    /// Check if a number is even and store it as last processed number.
    pub fn is_even(&mut self, number: i32) -> bool {
        self.number = number;
        match self.number % 2 {
            0 => true,
            _ => false,
        }
    }
}

/// Run entry point for the main thread.
#[wasm_bindgen]
pub fn startup() {
    // This is not strictly needed but makes debugging a lot easier.
    // Should not be used in productive deployments.
    set_panic_hook();

    // Here, we create our worker. In a larger app, multiple callbacks should be able to interact
    // with the code in the worker. Therefore, we wrap it in `Rc<RefCell>` following the interior
    // mutability pattern. In this example, it would not be needed but we include the wrapping
    // anyway as example.
    let worker_handle1 = Rc::new(RefCell::new(Worker::new("./worker.js").unwrap()));
    console::log_1(&"Created a new worker from within WASM".into());

    let worker_handle2 = Rc::new(RefCell::new(Worker::new("./worker.js").unwrap()));
    console::log_1(&"Created a new worker from within WASM".into());

    // Pass the worker to the function which sets up the `onchange` callback.
    setup_input_onchange_callback(worker_handle1);
    setup_button_click_callback(worker_handle2);
}

fn generate_data(width: usize, height: usize) -> Vec<Vec<String>> {
    console::log_1(&"generating data".into());
    let mut data = Vec::new();
    for _ in 0..height {
        let mut row = Vec::new();
        for _ in 0..width {
            let mut str = String::new();
            for _ in 0..10 {
                str.push(qrand::gen_range(64, 128).into());
            }
            row.push(str)
        }
        data.push(row)
    }
    console::log_1(&"generated data".into());
    data
}

// no generic type parameter, no func
fn group_by(group_data: Vec<Vec<String>>, num: usize) -> Vec<Vec<Vec<String>>> {
    let mut map: HashMap<&str, Vec<Vec<String>>> = HashMap::new();
    group_data.iter().for_each(|v| {
        map.entry(v[num].as_str())
            .or_insert(Vec::new())
            .push(v.to_owned());
    });
    map.into_values().map(|v| v.into_iter().collect()).collect()
}

fn setup_button_click_callback(_worker: Rc<RefCell<Worker>>) {
    let document = web_sys::window().unwrap().document().unwrap();

    #[allow(unused_assignments)]
    let mut _persistent_callback_handle = get_on_msg_callback();

    let callback = Closure::wrap(Box::new(move || {
        console::log_1(&"button click callback triggered".into());

        let groupby_data = generate_data(100, 10000);
        let res = group_by(groupby_data, 0);

        console::log_1(&format!("{:?}", res).as_str().into());
    }) as Box<dyn FnMut()>);

    document
        .get_element_by_id("start")
        .expect("#start should exist")
        .dyn_ref::<HtmlElement>()
        .expect("#inputNumber should be a HtmlInputElement")
        .set_onclick(Some(callback.as_ref().unchecked_ref()));

    // Leaks memory.
    callback.forget();
}

fn setup_input_onchange_callback(worker: Rc<RefCell<web_sys::Worker>>) {
    let document = web_sys::window().unwrap().document().unwrap();

    // If our `onmessage` callback should stay valid after exiting from the `onchange` closure,
    // we need to either forget it (so it is not destroyed) or store it somewhere.
    // To avoid leaking memory every time we want to receive a response from the worker, we
    // move a handle into the `onchange` closure to which we will always attach the last `onmessage`
    // callback. The initial value will not be used and we silence the warning.
    #[allow(unused_assignments)]
    let mut persistent_callback_handle = get_on_msg_callback();

    let callback = Closure::wrap(Box::new(move || {
        console::log_1(&"onchange callback triggered".into());
        let document = web_sys::window().unwrap().document().unwrap();

        let input_field = document
            .get_element_by_id("inputNumber")
            .expect("#inputNumber should exist");
        let input_field = input_field
            .dyn_ref::<HtmlInputElement>()
            .expect("#inputNumber should be a HtmlInputElement");

        // If the value in the field can be parsed to a `i32`, send it to the worker. Otherwise
        // clear the result field.
        match input_field.value().parse::<i32>() {
            Ok(number) => {
                // Access worker behind shared handle, following the interior mutability pattern.
                let worker_handle = &*worker.borrow();
                let _ = worker_handle.post_message(&number.into());
                persistent_callback_handle = get_on_msg_callback();

                // Since the worker returns the message asynchronously, we attach a callback to be
                // triggered when the worker returns.
                worker_handle
                    .set_onmessage(Some(persistent_callback_handle.as_ref().unchecked_ref()));
            }
            Err(_) => {
                document
                    .get_element_by_id("resultField")
                    .expect("#resultField should exist")
                    .dyn_ref::<HtmlElement>()
                    .expect("#resultField should be a HtmlInputElement")
                    .set_inner_text("");
            }
        }
    }) as Box<dyn FnMut()>);

    // Attach the closure as `onchange` callback to the input field.
    document
        .get_element_by_id("inputNumber")
        .expect("#inputNumber should exist")
        .dyn_ref::<HtmlInputElement>()
        .expect("#inputNumber should be a HtmlInputElement")
        .set_oninput(Some(callback.as_ref().unchecked_ref()));

    // Leaks memory.
    callback.forget();
}

/// Create a closure to act on the message returned by the worker
fn get_on_msg_callback() -> Closure<dyn FnMut(MessageEvent)> {
    let callback = Closure::wrap(Box::new(move |event: MessageEvent| {
        console::log_2(&"Received response: ".into(), &event.data().into());

        let result = match event.data().as_bool().unwrap() {
            true => "even",
            false => "odd",
        };

        let document = web_sys::window().unwrap().document().unwrap();
        document
            .get_element_by_id("resultField")
            .expect("#resultField should exist")
            .dyn_ref::<HtmlElement>()
            .expect("#resultField should be a HtmlInputElement")
            .set_inner_text(result);
    }) as Box<dyn FnMut(_)>);

    callback
}

/// Set a hook to log a panic stack trace in JS.
pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
