# On colorscheme changed

Rust-based service to listen on DBUS for color scheme changing (e.g. when toggling light/dark mode in the menubar in Gnome 42+),
and updating the theme for terminal emulators / terminal editors accordingly.

## Usage

```sh
git clone <this repo>
cd on-colorscheme-changed
# open src/main.rs in your favorite editor and adjust the `on_colorscheme_changed` function
# to your liking.

# If you want to run it in the foreground to try it out
cargo run

# If you want to run it in the background with systemd
cargo install --locked --path . # also ensure ~/.cargo/bin is in $PATH
systemctl --user enable ./on-colorscheme-changed.service
# The above command only works nicely on Fedora 39.
# Otherwise you can copy or symlink it to ~/.config/systemd/user
# See https://wiki.archlinux.org/title/Systemd/User 

systemctl --user start on-colorscheme-changed
```

## Development

// TODO
