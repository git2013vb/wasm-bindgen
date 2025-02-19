#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = WritableStream , typescript_type = "WritableStream")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WritableStream` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStream`*"]
    pub type WritableStream;
    # [wasm_bindgen (structural , method , getter , js_class = "WritableStream" , js_name = locked)]
    #[doc = "Getter for the `locked` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStream/locked)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStream`*"]
    pub fn locked(this: &WritableStream) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(catch, constructor, js_class = "WritableStream")]
    #[doc = "The `new WritableStream(..)` constructor, creating a new instance of `WritableStream`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStream/WritableStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStream`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Result<WritableStream, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(catch, constructor, js_class = "WritableStream")]
    #[doc = "The `new WritableStream(..)` constructor, creating a new instance of `WritableStream`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStream/WritableStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStream`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new_with_underlying_sink(
        underlying_sink: &::js_sys::Object,
    ) -> Result<WritableStream, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "QueuingStrategy")]
    #[wasm_bindgen(catch, constructor, js_class = "WritableStream")]
    #[doc = "The `new WritableStream(..)` constructor, creating a new instance of `WritableStream`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStream/WritableStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `QueuingStrategy`, `WritableStream`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new_with_underlying_sink_and_strategy(
        underlying_sink: &::js_sys::Object,
        strategy: &QueuingStrategy,
    ) -> Result<WritableStream, JsValue>;
    # [wasm_bindgen (method , structural , js_class = "WritableStream" , js_name = abort)]
    #[doc = "The `abort()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStream/abort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStream`*"]
    pub fn abort(this: &WritableStream) -> ::js_sys::Promise;
    # [wasm_bindgen (method , structural , js_class = "WritableStream" , js_name = abort)]
    #[doc = "The `abort()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStream/abort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStream`*"]
    pub fn abort_with_reason(
        this: &WritableStream,
        reason: &::wasm_bindgen::JsValue,
    ) -> ::js_sys::Promise;
    # [wasm_bindgen (method , structural , js_class = "WritableStream" , js_name = close)]
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStream/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStream`*"]
    pub fn close(this: &WritableStream) -> ::js_sys::Promise;
    #[cfg(feature = "WritableStreamDefaultWriter")]
    # [wasm_bindgen (method , structural , js_class = "WritableStream" , js_name = getWriter)]
    #[doc = "The `getWriter()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStream/getWriter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStream`, `WritableStreamDefaultWriter`*"]
    pub fn get_writer(this: &WritableStream) -> WritableStreamDefaultWriter;
}
