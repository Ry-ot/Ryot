use leafwing_input_manager::user_input::Modifier;
use rfd::AsyncFileDialog;
use ryot::helpers::execute;

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
