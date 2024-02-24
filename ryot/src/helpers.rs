use std::future::Future;

#[cfg(not(target_arch = "wasm32"))]
pub fn execute_async<F: Future<Output = ()> + Send + 'static>(f: F) {
    async_std::task::spawn(f);
}

#[cfg(target_arch = "wasm32")]
pub fn execute<F: Future<Output = ()> + 'static>(f: F) {
    wasm_bindgen_futures::spawn_local(f);
}
