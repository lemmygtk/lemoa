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
* Logging in with an account

Not yet working, but planned to be added soon:
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

# License
Lemoa is licensed under the [**GNU General Public License**](https://www.gnu.org/licenses/gpl.html): You can use, study and share it as you want.
