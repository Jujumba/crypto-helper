use js_sys::Function;
use wasm_bindgen::JsValue;
use web_sys::MouseEvent;
use yew::{
    classes, function_component, html, use_effect_with_deps, use_state, Callback, Classes, Html, Properties,
    UseStateSetter,
};
use yew_notifications::{Notification, NotificationType};

use crate::utils::gen_copy_func;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum BytesFormat {
    Hex,
    Base64,
    Ascii,
}

impl From<&BytesFormat> for &str {
    fn from(format: &BytesFormat) -> Self {
        match format {
            BytesFormat::Hex => "hex",
            BytesFormat::Base64 => "base64",
            BytesFormat::Ascii => "ascii",
        }
    }
}

pub const BYTES_FORMATS: [BytesFormat; 3] = [BytesFormat::Hex, BytesFormat::Base64, BytesFormat::Ascii];

fn encode_bytes(bytes: &[u8], format: &BytesFormat) -> String {
    match format {
        BytesFormat::Hex => hex::encode(bytes),
        BytesFormat::Base64 => base64::encode(bytes),
        BytesFormat::Ascii => bytes.iter().map(|c| *c as char).collect(),
    }
}

#[derive(PartialEq, Properties, Clone)]
pub struct SimpleOutputProps {
    output: Vec<u8>,
    format: BytesFormat,
    add_notification: Callback<Notification>,
}

fn get_set_format_callback(format: BytesFormat, set_format: UseStateSetter<BytesFormat>) -> Callback<MouseEvent> {
    Callback::from(move |_event| {
        set_format.set(format);
    })
}

fn get_format_button_class(selected: bool) -> Classes {
    if selected {
        classes!("format-button", "format-button-selected")
    } else {
        classes!("format-button")
    }
}

#[function_component(SimpleOutput)]
pub fn simple_output(props: &SimpleOutputProps) -> Html {
    let SimpleOutputProps {
        output,
        format,
        add_notification,
    } = props.clone();

    let bytes_format = use_state(|| format);

    let format_setter = bytes_format.setter();
    use_effect_with_deps(
        move |format| {
            format_setter.set(**format);
        },
        bytes_format.clone(),
    );

    let encoded_bytes = encode_bytes(&output, &bytes_format);

    let encoded = encoded_bytes.clone();
    let onclick = Callback::from(move |_| {
        let function = Function::new_no_args(&gen_copy_func(&encoded));
        if function.call0(&JsValue::null()).is_ok() {
            add_notification.emit(Notification::from_description_and_type(
                NotificationType::Info,
                "output copied",
            ))
        }
    });

    html! {
        <div class={classes!("output")}>
            <div class={classes!("formats-container")}>{
                BYTES_FORMATS.iter().map(|format| {
                    html! {
                        <button
                            class={get_format_button_class(*bytes_format == *format)}
                            onclick={get_set_format_callback(*format, bytes_format.setter())}
                        >
                            {<&str>::from(format)}
                        </button>
                    }
                }).collect::<Html>()
            }</div>
            <span class={classes!("simple-digest")} onclick={onclick}>{encoded_bytes}</span>
            <span class={classes!("total")}>{format!("total: {}", output.len())}</span>
        </div>
    }
}

pub fn build_simple_output(output: Vec<u8>, format: BytesFormat, add_notification: Callback<Notification>) -> Html {
    html! {
        <SimpleOutput {output} {format} {add_notification} />
    }
}
