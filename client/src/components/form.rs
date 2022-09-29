use regex::Regex;
use std::collections::HashMap;
use web_sys::{Event, HtmlInputElement};
use yew::events::KeyboardEvent;
use yew::{classes, function_component, html, Callback, Properties, TargetCast};

#[derive(PartialEq, Properties, Clone)]
pub struct InputsProps {
    pub on_change: Callback<(String, String)>,
    pub inputs: HashMap<String, String>,
}

#[function_component(Form)]
pub fn form(props: &InputsProps) -> Html {
    let generate_onkeyup = |key: &str| {
        let on_change = props.on_change.clone();
        let key_string = key.to_string();

        move |e: KeyboardEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            on_change.emit((key_string.clone(), value));
        }
    };

    let generate_onselect = |key: &str| {
        let on_change = props.on_change.clone();
        let key_string = key.to_string();

        move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            on_change.emit((key_string.clone(), value));
        }
    };

    let default_value = "".to_string();
    let input_value = |key: &str| {
        let input_key = key.to_string();
        props
            .inputs
            .get(&input_key)
            .unwrap_or(&default_value)
            .clone()
    };

    let subject_options = vec![
        ("0", "Question about a product"),
        ("1", "Question about a owner"),
        ("2", "Another"),
    ];
    let email_regex = Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
    )
    .unwrap();
    let is_valid_email = |email: String| email_regex.is_match(email.as_str());

    html! {
        <div class="my-3">
            <div class="my-1">
                <label class="label">{ "Title" }</label>
                <div class="control">
                    <input
                        class="input"
                        placeholder="Title"
                        onkeyup = {generate_onkeyup("title")}
                        value = {input_value("title")}
                    />
                </div>
            </div>
            <div class="my-1">
                <label class="label">{ "Username" }</label>
                <div class="control has-icons-left has-icons-right">
                    <input
                        class="input"
                        type="text"
                        placeholder="Tanaka Taro"
                        onkeyup = {generate_onkeyup("username")}
                        value = {input_value("username")}
                    />
                    <span class="icon is-small is-left">
                        <i class="fas fa-user"></i>
                    </span>
                </div>
            </div>
            <div class="my-1">
                <label class="label">{ "Email" }</label>
                <div class="control has-icons-left has-icons-right">
                    <input
                        class={classes!(
                            "input",
                            if is_valid_email(input_value("email")) { "" } else { "is-danger" },
                        )}
                        type="email"
                        placeholder="taro@example.com"
                        onkeyup = {generate_onkeyup("email")}
                        value = {input_value("email")}
                    />
                    <span class="icon is-small is-left">
                        <i class="fas fa-envelope"></i>
                    </span>
                </div>
            </div>
            <div class="my-1">
                <label class="label">{ "Subject" }</label>
                <div class="control">
                    <div class="select">
                        <select onchange={generate_onselect("subject")}>
                            {
                                subject_options.into_iter().map(|so| {
                                    html!{<option value={so.0.to_string()}>{ so.1 }</option>}
                                }).collect::<html::Html>()
                            }
                        </select>
                    </div>
                </div>
            </div>
            <div class="my-1">
                <label class="label">{ "Message" }</label>
                <div class="control">
                    <textarea
                        class="textarea"
                        placeholder="Message"
                        onkeyup = {generate_onkeyup("message")}
                        value = {input_value("message")}
                    />
                </div>
            </div>
        </div>
    }
}
