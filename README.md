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
* Writing posts or comments

Not yet working, but planned to be added soon:
* Deleting or upvoting posts or comments
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

# Building with Docker
```
sudo docker build --no-cache . -t lemoa:latest
CONTAINER_ID=$(sudo docker create --name lemoa -t lemoa:latest)
sudo docker cp $(CONTAINER_ID):/root/lemoa/target/release/lemoa .
```
Once the build is done, there will be an executable `lemoa` binary file in your current directory, executing it starts Lemoa :tada:.

# License
Lemoa is licensed under the [**GNU General Public License**](https://www.gnu.org/licenses/gpl.html): You can use, study and share it as you want.
