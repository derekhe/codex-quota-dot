use auto_launch::{AutoLaunch, AutoLaunchBuilder};
use std::path::Path;

pub fn manager() -> Result<AutoLaunch, Box<dyn std::error::Error>> {
    for_executable(&std::env::current_exe()?)
}

fn for_executable(path: &Path) -> Result<AutoLaunch, Box<dyn std::error::Error>> {
    let path = path
        .to_str()
        .ok_or("The executable path is not valid UTF-8")?
        .to_owned();
    // auto-launch writes this string directly into the Windows Run command.
    #[cfg(windows)]
    let path = format!("\"{path}\"");

    Ok(AutoLaunchBuilder::new()
        .set_app_name("Codex Quota Dot")
        .set_app_path(&path)
        .set_use_launch_agent(true)
        .build()?)
}

#[cfg(all(test, windows))]
mod tests {
    use super::*;

    #[test]
    #[ignore = "Changes this user's startup registration; requires CODEX_QUOTA_DOT_INSTALLED_EXE and leaves autostart enabled"]
    fn windows_installed_autostart_lifecycle() {
        let executable = std::env::var_os("CODEX_QUOTA_DOT_INSTALLED_EXE")
            .expect("Set CODEX_QUOTA_DOT_INSTALLED_EXE to the installed app");
        let executable = Path::new(&executable);
        assert!(executable.is_file());
        assert_eq!(executable.file_name().unwrap(), "codex-quota-dot.exe");
        let manager = for_executable(executable).unwrap();

        manager.enable().unwrap();
        assert!(manager.is_enabled().unwrap());
        manager.disable().unwrap();
        assert!(!manager.is_enabled().unwrap());
        manager.enable().unwrap();
        assert!(manager.is_enabled().unwrap());
    }

    #[test]
    fn windows_startup_quotes_executable_paths_with_spaces_and_unicode() {
        let manager = for_executable(Path::new(
            r"C:\Users\测试用户\My Apps\Codex Quota Dot\codex-quota-dot.exe",
        ))
        .unwrap();
        assert_eq!(manager.get_app_name(), "Codex Quota Dot");
        assert_eq!(
            manager.get_app_path(),
            r#""C:\Users\测试用户\My Apps\Codex Quota Dot\codex-quota-dot.exe""#
        );
        assert!(manager.get_args().is_empty());
    }
}
