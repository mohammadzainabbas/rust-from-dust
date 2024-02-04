async fn groot() -> Html<&'static str> {
    trace!("inside groot()");
    Html("Hello, I'm groot!")
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    pub name: Option<String>,
}

#[tracing::instrument]
async fn say_hello(Query(param): Query<HelloParams>) -> Response {
    trace!("inside say_hello()");
    let name = param.name.as_deref().unwrap_or("World");
    debug!(target: "say", name);
    Html(format!("<h3> Hello {}! </h3>", name)).into_response()
}

#[tracing::instrument]
async fn say_path(Path(path): Path<String>) -> impl IntoResponse {
    trace!("inside say_path()");

    Html(format!("<h3> Hello {}! </h3>", path.as_str())).into_response()
}
