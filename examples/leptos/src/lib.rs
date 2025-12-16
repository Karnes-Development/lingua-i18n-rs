use leptos::*;
use lingua_i18n_rs::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn main() {
    console_error_panic_hook::set_once();
    
    mount_to_body(|| {
        view! {
            <App />
        }
    });
}

#[component]
fn App() -> impl IntoView {
    let (initialized, set_initialized) = create_signal(false);
    let (current_lang, set_current_lang) = create_signal("en".to_string());
    
    spawn_local(async move {
        match Lingua::new("/languages")
            .with_languages(vec!["en".to_string(), "de".to_string()])
            .init()
            .await
        {
            Ok(_) => {
                set_initialized.set(true);
                
                Lingua::on_language_change(move |lang| {
                    set_current_lang.set(lang.to_string());
                });
                
                if let Ok(lang) = Lingua::get_language() {
                    set_current_lang.set(lang);
                }
            }
            Err(e) => {
                web_sys::console::error_1(&JsValue::from_str(&format!("Failed to initialize Lingua: {}", e)));
            }
        }
    });
    
    view! {
        <div class="app">
            <header>
                <h1>"🌍 Lingua i18n-rs + Leptos Example"</h1>
                <Show when=move || initialized.get()>
                    <LanguageSwitcher current_lang=current_lang set_current_lang=set_current_lang />
                </Show>
            </header>
            
            <main>
                <Show
                    when=move || initialized.get()
                    fallback=|| view! { <p>"Loading translations..."</p> }
                >
                    <Translations current_lang=current_lang />
                </Show>
            </main>
        </div>
    }
}

#[component]
fn LanguageSwitcher(
    current_lang: ReadSignal<String>,
    set_current_lang: WriteSignal<String>,
) -> impl IntoView {
    let languages = move || {
        Lingua::get_languages().unwrap_or_default()
    };
    
    let switch_language = move |lang: String| {
        if let Ok(_) = Lingua::set_language(&lang) {
            set_current_lang.set(lang);
        }
    };
    
    view! {
        <div class="language-switcher">
            <label>"Select Language: "</label>
            <select
                value=move || current_lang.get()
                on:change=move |ev| {
                    let lang = event_target_value(&ev);
                    switch_language(lang);
                }
            >
                {move || {
                    languages().into_iter().map(|lang| {
                        let is_selected = lang == current_lang.get();
                        view! {
                            <option selected=is_selected value=lang.clone()>
                                {lang}
                            </option>
                        }
                    }).collect::<Vec<_>>()
                }}
            </select>
        </div>
    }
}

#[component]
fn Translations(current_lang: ReadSignal<String>) -> impl IntoView {
    let welcome = move || {
        let _ = current_lang.get();
        Lingua::t("welcome", &[]).unwrap_or_else(|_| "Translation not found".to_string())
    };
    
    let greeting = move || {
        let _ = current_lang.get();
        Lingua::t("greeting", &[("name", "Leptos User")]).unwrap_or_else(|_| "Translation not found".to_string())
    };
    
    let save = move || {
        let _ = current_lang.get();
        Lingua::t("menu.file.save", &[]).unwrap_or_else(|_| "Translation not found".to_string())
    };
    
    let open = move || {
        let _ = current_lang.get();
        Lingua::t("menu.file.open", &[]).unwrap_or_else(|_| "Translation not found".to_string())
    };
    
    view! {
        <div class="translations">
            <h2>"Current Language: " {move || current_lang.get()}</h2>
            
            <div class="translation-item">
                <strong>"Welcome:"</strong>
                <p>{welcome}</p>
            </div>
            
            <div class="translation-item">
                <strong>"Greeting:"</strong>
                <p>{greeting}</p>
            </div>
            
            <div class="translation-item">
                <strong>"Menu Items:"</strong>
                <ul>
                    <li>"Open: " {open}</li>
                    <li>"Save: " {save}</li>
                </ul>
            </div>
        </div>
    }
}
