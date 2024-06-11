#![cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
///
/// Wasm Asset Reader with appending `?v=xxx` to asset's path
///
/// almost all codes copied from bevy_asset (src/io/wasm.rs)
///
///
use bevy::prelude::*;
use bevy::asset::io::{
    /*get_meta_path,*/
    AssetReader, AssetReaderError, 
    /* EmptyPathStream, */ 
    PathStream, Reader, VecReader,

    AssetSourceId, AssetSource,
};
use bevy::log::error;
use bevy::utils::BoxedFuture;
use js_sys::{Uint8Array, JSON};
use std::path::{Path, PathBuf};
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::Response;
use futures_lite::Stream;

pub struct HttpWithVersionQueryStringWasmAssetReaderPlugin {
    version: &'static str,
}
impl HttpWithVersionQueryStringWasmAssetReaderPlugin {
    pub fn new(version: &'static str) -> Self {
        Self { version }
    }
}
impl Plugin for HttpWithVersionQueryStringWasmAssetReaderPlugin {
    fn build(&self, app: &mut App) {
        app.register_asset_source(
            AssetSourceId::Default,
            AssetSource::build().with_reader(|| {
                Box::new(HttpWithVersionQueryStringWasmAssetReader::new("assets".to_string(), self.version)
                )
            }),
        );
    }
}

fn get_meta_path(path: &Path) -> PathBuf {
    let mut path_buf = path.to_path_buf();
    let mut extension = path_buf.extension().unwrap_or_default().to_os_string();
    extension.push(".meta");
    path_buf.set_extension(extension);
    path_buf
}

struct EmptyPathStream;
impl Stream for EmptyPathStream {
    type Item = PathBuf;
    fn poll_next(self: std::pin::Pin<&mut Self>, _cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>> {
        std::task::Poll::Ready(None)
    }
}

/// Represents the global object in the JavaScript context
#[wasm_bindgen]
extern "C" {
    /// The [Global](https://developer.mozilla.org/en-US/docs/Glossary/Global_object) object.
    type Global;

    /// The [window](https://developer.mozilla.org/en-US/docs/Web/API/Window) global object.
    #[wasm_bindgen(method, getter, js_name = Window)]
    fn window(this: &Global) -> JsValue;

    /// The [WorkerGlobalScope](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope) global object.
    #[wasm_bindgen(method, getter, js_name = WorkerGlobalScope)]
    fn worker(this: &Global) -> JsValue;
}

/// Reader implementation for loading assets via HTTP in WASM.
struct HttpWithVersionQueryStringWasmAssetReader {
    root_path: PathBuf,

    version: String,
}

impl HttpWithVersionQueryStringWasmAssetReader {
    /// Creates a new `WasmAssetReader`. The path provided will be used to build URLs to query for assets.
    pub fn new<P: AsRef<Path>>(path: P, version: &str) -> Self {
        Self {
            root_path: path.as_ref().to_owned(),
            version: version.to_string(),
        }
    }
}

fn js_value_to_err<'a>(context: &'a str) -> impl FnOnce(JsValue) -> std::io::Error + 'a {
    move |value| {
        let message = match JSON::stringify(&value) {
            Ok(js_str) => format!("Failed to {context}: {js_str}"),
            Err(_) => {
                format!("Failed to {context} and also failed to stringify the JSValue of the error")
            }
        };

        std::io::Error::new(std::io::ErrorKind::Other, message)
    }
}

impl HttpWithVersionQueryStringWasmAssetReader {
    async fn fetch_bytes<'a>(&self, path: PathBuf) -> Result<Box<Reader<'a>>, AssetReaderError> {
        // The JS global scope includes a self-reference via a specialising name, which can be used to determine the type of global context available.
        let global: Global = js_sys::global().unchecked_into();
        let promise = if !global.window().is_undefined() {
            let window: web_sys::Window = global.unchecked_into();
            window.fetch_with_str(path.to_str().unwrap())
        } else if !global.worker().is_undefined() {
            let worker: web_sys::WorkerGlobalScope = global.unchecked_into();
            worker.fetch_with_str(path.to_str().unwrap())
        } else {
            let error = std::io::Error::new(
                std::io::ErrorKind::Other,
                "Unsupported JavaScript global context",
            );
            return Err(AssetReaderError::Io(error.into()));
        };
        let resp_value = JsFuture::from(promise)
            .await
            .map_err(js_value_to_err("fetch path"))?;
        let resp = resp_value
            .dyn_into::<Response>()
            .map_err(js_value_to_err("convert fetch to Response"))?;
        match resp.status() {
            200 => {
                let data = JsFuture::from(resp.array_buffer().unwrap()).await.unwrap();
                let bytes = Uint8Array::new(&data).to_vec();
                let reader: Box<Reader> = Box::new(VecReader::new(bytes));
                Ok(reader)
            }
            404 => Err(AssetReaderError::NotFound(path)),
            status => Err(AssetReaderError::HttpError(status as u16)),
        }
    }
}

impl HttpWithVersionQueryStringWasmAssetReader {
    fn get_path_with_v(&self, path: &Path) -> PathBuf {
        let mut path_buf = path.to_path_buf();
        let mut ext = path.extension().unwrap_or_default().to_os_string();
        ext.push(format!("?v={}", self.version));
        path_buf.set_extension(ext);
        path_buf
    }
}

impl AssetReader for HttpWithVersionQueryStringWasmAssetReader {
    fn read<'a>(
        &'a self,
        path: &'a Path,
    ) -> BoxedFuture<'a, Result<Box<Reader<'a>>, AssetReaderError>> {
        Box::pin(async move {
            let path = self.root_path.join(path);
            let path = self.get_path_with_v(path.as_path());
            self.fetch_bytes(path).await
        })
    }

    fn read_meta<'a>(
        &'a self,
        path: &'a Path,
    ) -> BoxedFuture<'a, Result<Box<Reader<'a>>, AssetReaderError>> {
        Box::pin(async move {
            let meta_path = get_meta_path(&self.root_path.join(path));
            let meta_path = self.get_path_with_v(meta_path.as_path());
            Ok(self.fetch_bytes(meta_path).await?)
        })
    }

    fn read_directory<'a>(
        &'a self,
        _path: &'a Path,
    ) -> BoxedFuture<'a, Result<Box<PathStream>, AssetReaderError>> {
        let stream: Box<PathStream> = Box::new(EmptyPathStream);
        error!("Reading directories is not supported with the HttpWithVersionQueryStringWasmAssetReader");
        Box::pin(async move { Ok(stream) })
    }

    fn is_directory<'a>(
        &'a self,
        _path: &'a Path,
    ) -> BoxedFuture<'a, std::result::Result<bool, AssetReaderError>> {
        error!("Reading directories is not supported with the HttpWithVersionQueryStringWasmAssetReader");
        Box::pin(async move { Ok(false) })
    }
}

