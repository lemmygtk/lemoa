# lemoa

Native Gtk client for Lemmy (beta)

## Installation

| Platform | Command                                                                                                                                                 |
| -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Flathub  | <a href="https://flathub.org/apps/details/io.github.lemmygtk.lemoa"><img src="https://flathub.org/assets/badges/flathub-badge-en.png" width="150"/></a> |

If you don't use any of these platforms, consider [building the app from source](#Building).

# Features

- Listing trending posts
- Viewing a post and its comments
- Viewing profiles
- Viewing trending communities
- Searching communities
- Logging in with an account
- Writing posts or comments
- Viewing subscribed posts
- Following communities
- Voting for or against posts or comments
- Editing and deleting posts or comments
- Viewing the personal inbox (mentions, replies)
- Private messages

# Build dependencies

- rust
- cargo
- pkg-config
- libgtk-4-dev or gtk4-devel (name depends on the distro)

# Building

## Building with meson

```
meson _build
ninja -C _build
sudo ninja -C _build install
```

## Building with Docker

```
sudo docker build --no-cache . -t lemoa:latest
CONTAINER_ID=$(sudo docker create --name lemoa -t lemoa:latest)
sudo docker cp $(CONTAINER_ID):/root/lemoa/target/release/lemoa .
```

Once the build is done, there will be an executable `lemoa` binary file in your current directory, executing it starts Lemoa :tada:.

## Building the binary only

Not recommended: To only install the binary (can only be started with the terminal), run

```sh
cargo install --git https://github.com/lemmygtk/lemoa.git
```

You can then start the app via the terminal after adding cargo's bin directory to your PATH variable.

# Development

Clone the repository and run

```sh
cargo run
```

# License

Lemoa is licensed under the [**GNU General Public License**](https://www.gnu.org/licenses/gpl.html): You can use, study and share it as you want.
