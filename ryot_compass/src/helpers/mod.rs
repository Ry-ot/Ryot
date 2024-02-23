use rfd::AsyncFileDialog;
use std::future::Future;

#[cfg(not(target_arch = "wasm32"))]
pub fn execute<F: Future<Output = ()> + Send + 'static>(f: F) {
    async_std::task::spawn(f);
}

#[cfg(target_arch = "wasm32")]
pub fn execute<F: Future<Output = ()> + 'static>(f: F) {
    wasm_bindgen_futures::spawn_local(f);
}

pub fn read_file(
    async_rfd: AsyncFileDialog,
    callback: impl FnOnce((String, Vec<u8>)) + 'static + Send,
) {
    let task = async_rfd.pick_file();

    execute(async {
        let file = task.await;

        if let Some(file) = file {
            callback((file.file_name(), file.read().await));
        }
    });
}

use leafwing_input_manager::user_input::Modifier;

#[cfg(target_os = "macos")]
pub static CONTROL_COMMAND: Modifier = Modifier::Super;

#[cfg(not(target_os = "macos"))]
pub static CONTROL_COMMAND: Modifier = Modifier::Control;

#[macro_export]
macro_rules! inputs {
    ( $($input:expr),* ) => {
        [
            $( InputKind::from($input) ),*
        ]
    };
}
