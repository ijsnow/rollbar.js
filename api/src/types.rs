use url::{ParseError, Url as BaseUrl};

pub struct Url {
    base: BaseUrl,
}

#[cfg(not(target_arch = "wasm32"))]
use neon::prelude::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

impl Url {
    pub fn new(raw: &str) -> Result<Self, ParseError> {
        let base = BaseUrl::parse(raw)?;

        Ok(Self { base })
    }

    fn hostname(&self) -> String {
        self.base.host_str().unwrap_or("api.rollbar.com").into()
    }

    fn pathname(&self) -> String {
        self.base.path().into()
    }

    fn protocol(&self) -> String {
        format!("{}:", self.base.scheme())
    }

    fn port(&self) -> u16 {
        self.base.port().unwrap_or(443)
    }

    fn query(&self) -> String {
        self.base.query().unwrap_or("").into()
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn to_object<'a>(&self, cx: &mut FunctionContext<'a>) -> JsResult<'a, JsObject> {
        let obj: Handle<JsObject> = cx.empty_object();

        let hostname = cx.string(self.hostname());
        obj.set(cx, "hostname", hostname)?;

        let pathname = cx.string(self.pathname());
        obj.set(cx, "pathname", pathname)?;

        let protocol = cx.string(self.protocol());
        obj.set(cx, "protocol", protocol)?;

        let port = cx.number(self.port());
        obj.set(cx, "port", port)?;

        let query = cx.string(self.query());
        obj.set(cx, "query", query)?;

        Ok(obj)
    }

    pub fn to_serializable<'a>(&self) -> impl serde::Serialize {
        #[derive(serde::Serialize)]
        struct Repr {
            hostname: String,
            pathname: String,
            port: u16,
            protocol: String,
            query: String,
        }

        Repr {
            hostname: self.hostname(),
            pathname: self.pathname(),
            protocol: self.protocol(),
            port: self.port(),
            query: self.query(),
        }
    }
}
