use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct Hello {
    a: String,
    b: String,
    c: String,
}

/// hello world GET API endpoint
#[server(HelloWorldAction, "/api", "GetJson")]
#[tracing::instrument]
pub async fn hello_world() -> Result<String, ServerFnError> {
    Ok("Hello World!".to_string())
}

/// Create hello world POST API endpoint
#[server(CreateHelloAction, "/api")]
#[tracing::instrument]
pub async fn create_hello(hello: Hello) -> Result<String, ServerFnError> {
    println!("REQUEST HELLO: {:#?}", hello);
    Ok("Hello World!".to_string())
}

#[component]
pub fn HelloWorld() -> impl IntoView {
    let async_data = create_resource(
        || (),
        |_| async move {
            logging::log!("Loading hello world data from API");
            hello_world().await
        },
    );
    view! {
        <h1>"Welcome to Hello World!"</h1>
        <Suspense fallback=|| ()>
            {move || match async_data.get() {
                Some(result) => match result {
                    Ok(result) => view!{<p>"Here is my message: "{result}</p>}.into_view(),
                    Err(_) => view!{<p>"Error loading hello world..."</p>}.into_view(),
                },
                None => view!{<p>"Loading hello world..."</p>}.into_view(),
            }}
        </Suspense>
        <Hello />
    }
}

#[component]
pub fn Hello() -> impl IntoView {
    let submit = create_server_action::<CreateHelloAction>();

    view! {
      <ActionForm action=submit>
        <input type="text" name="hello[a]"/>
        <input type="text" name="hello[b]"/>
        <input type="text" name="hello[c]"/>
        <input type="submit" text="Submit Hello"/>
      </ActionForm>
    }
}
