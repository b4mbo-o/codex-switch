mod import;
mod launch;
mod login;
mod misc;
mod profile;
mod render;
mod update;

pub(crate) use import::import_cmd;
pub(crate) use launch::launch_cmd;
pub(crate) use login::login_cmd;
pub(crate) use misc::{open_cmd, reset_card_cmd, warmup_cmd};
pub(crate) use profile::{delete_cmd, list_cmd, rename_cmd, use_cmd};
pub(crate) use render::confirm;
pub(crate) use update::self_update_cmd;

/// A shared Codex app-server can keep the account it loaded before auth.json
/// changed. Older Codex releases do not recognize this opt-out flag.
fn codex_supports_no_daemon() -> bool {
    std::process::Command::new("codex")
        .arg("--help")
        .output()
        .ok()
        .filter(|output| output.status.success())
        .is_some_and(|output| String::from_utf8_lossy(&output.stdout).contains("--no-daemon"))
}
