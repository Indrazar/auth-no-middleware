use leptos::prelude::*;
//#[cfg(debug_assertions)]
//use leptos_router::ActionForm;

#[cfg(feature = "ssr")]
use http::request::Parts;

/// Renders a button that sends a post request to /api
/// On the server side this will print out all the headers provided by the client
#[component]
pub fn LogHeader() -> impl IntoView {
    #[cfg(debug_assertions)]
    let log_header = ServerAction::<LogClientHeader>::new();

    #[cfg(debug_assertions)]
    view! {
        <p>
            <ActionForm action=log_header>
                <button class="log_header" type="submit">"Log Current Headers"</button>
            </ActionForm>
        </p>
    }
}

//debugging tool
#[server(LogClientHeader, "/api")]
async fn log_client_headers() -> Result<String, ServerFnError> {
    // this is just an example of how to access server context injected in the handlers
    let http_req = use_context::<Parts>();
    if let Some(http_req) = http_req {
        log::debug!(
            "Client pressed LogHeader, printing all data from client:\nhttp_req.version: \
             {:#?}\nhttp_req.method: {:#?}\nhttp_req.uri.path(): {:#?}\nhttp_req.headers: \
             {:#?}\nhttp_req.extensions: {:#?}",
            &http_req.version,
            &http_req.method,
            &http_req.uri.path(),
            &http_req.headers,
            &http_req.extensions
        );
        // ResponseOptions are more of an outbox than incoming data
        //log::debug!("resp_opt: {:#?}", use_context::<leptos_actix::ResponseOptions>(cx));
        //log::debug!(
        //    "route_int_ctx: {:#?}",
        //    use_context::<leptos_router::RouterIntegrationContext>()
        //);
        log::debug!("meta_ctx: {:#?}", use_context::<leptos_meta::MetaContext>());
    }

    Ok("It worked".to_string())
}
