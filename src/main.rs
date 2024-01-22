use futures_util::stream::StreamExt;
use std::{error::Error, rc::Rc};
use tokio::fs;

use zbus::{dbus_proxy, zvariant::OwnedValue, Connection};

#[dbus_proxy(
    interface = "org.freedesktop.portal.Settings",
    default_service = "org.freedesktop.portal.Desktop",
    default_path = "/org/freedesktop/portal/desktop"
)]
trait Settings {
    #[dbus_proxy(signal)]
    fn setting_changed(namespace: &str, key: &str, value: OwnedValue) -> zbus::Result<()>;
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum ColorScheme {
    Light,
    Dark,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let conn = Connection::session().await?;
    let settings = SettingsProxy::new(&conn).await?;
    let mut settings_changed = settings
        .receive_setting_changed_with_args(&[
            (0, "org.freedesktop.appearance"),
            (1, "color-scheme"),
        ])
        .await?;

    println!("Connected to dbus! Waiting for changes...");

    while let Some(setting) = settings_changed.next().await {
        if let Ok(args) = setting.args() {
            let val = TryInto::<u32>::try_into(args.value)?;
            match val {
                0 => on_colorscheme_changed(ColorScheme::Light).await?,
                1 => on_colorscheme_changed(ColorScheme::Dark).await?,
                _ => panic!("Unexpected value"),
            };
        } else {
            eprintln!("{:?}", setting.args());
        }
    }

    Ok(())
}

async fn on_colorscheme_changed(cs: ColorScheme) -> Result<(), Box<dyn Error>> {
    println!("Colorscheme changed: {:?}", cs);

    let (from, to) = if cs == ColorScheme::Light {
        ("catppuccin_mocha", "catppuccin_latte")
    } else {
        ("catppuccin_latte", "catppuccin_mocha")
    };

    let mut helix_conf_file = dirs::config_dir().unwrap();
    helix_conf_file.push("helix/config.toml");
    let helix_conf = fs::read_to_string(&helix_conf_file).await?;

    if helix_conf.contains(from) {
        println!("Updating helix config to {:?}", cs);
        let helix_conf = helix_conf.replace(from, to);
        fs::write(helix_conf_file, &helix_conf).await?;
    } else {
        println!("helix config did not contain {from}. Nothing to do.");
    }

    let mut from_fname = dirs::config_dir().unwrap();
    let mut to_fname = dirs::config_dir().unwrap();
    if cs == ColorScheme::Light {
        from_fname.push("alacritty/alacritty.light.toml");
        to_fname.push("alacritty/alacritty.dark.toml");
    } else {
        from_fname.push("alacritty/alacritty.dark.toml");
        to_fname.push("alacritty/alacritty.light.toml");
    };

    if fs::try_exists(&from_fname).await.is_ok_and(|val| val) {
        println!("Updating alacritty.toml to {:?}", cs);
        let mut alacritty_conf_file = dirs::config_dir().unwrap();
        alacritty_conf_file.push("alacritty/alacritty.toml");
        fs::rename(&alacritty_conf_file, &to_fname).await?;
        fs::rename(&from_fname, &alacritty_conf_file).await?;
    } else {
        println!("File {from_fname:?} not found. Nothing to do.");
    }

    Ok(())
}
