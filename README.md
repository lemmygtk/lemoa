# lemoa
Native Gtk client for Lemmy (alpha state)

# Current state
Under development, not yet ready for daily usage
Working:
* Selecting lemmy instance
* Listing trending posts
* Viewing a post and its comments
* Viewing profiles
* Viewing trending communities
* Searching communities

Not yet working:
* Logging in
* Writing posts or comments
* Insteracting with posts or comments
* Private messages
* Viewing the next pages of posts or communities

# Build dependencies
* rust
* cargo
* libgtk-4-dev

# Installation
```sh
cargo install --git https://github.com/lemmy-gtk/lemoa.git
```
You can then start the app via the terminal after adding cargo's bin directory to your PATH variable
```sh
lemoa
```

# Development
Clone the repository and run
```sh
cargo run
```

# Build inside Docker
Be sure you to replace `$PATH_TO_REPO` with the path to this cloned repo on your filesystem.
```
sudo docker build --no-cache . -t lemoa:latest
sudo docker run -v $PATH_TO_REPO:/home/lemoa/lemoa_volume -it lemoa:latest bash
cd lemoa_volume/
cargo build
```
The binary will also be available on your host system since $PATH_TO_REPO is mounted as a shared volume between docker and host. 


# License
Lemoa is licensed under the [**GNU General Public License**](https://www.gnu.org/licenses/gpl.html): You can use, study and share it as you want.
