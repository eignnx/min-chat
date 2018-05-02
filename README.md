# min-chat
A minimal (or at least very small) chat client/server implementation written in Rust.

## Instructions for Running
1. Run the server with:
  ```shell
  $ cargo run
  ```
  
2. Serve client somehow. I'm using Python's `SimpleHTTPServer`:
  ```shell
  $ cd src/client
  $ python -m SimpleHTTPServer 8080
  ```
  
3. Go to `localhost:8080/index.html` in your browser.

## The Plan
* [ ] Javascript clients run in browser
  * [ ] Direct new users to login page
  * [ ] Listen for new chat messages sent from server, updates client's chat history
  * [ ] Allow users to type messages while new ones are being recieved simultaneously
  * [X] Send new chat messages to the server over the WebSocket
* [ ] Rust-lang server
  * [X] Listens for new client connnections, spawns thread to handle each new client
  * [ ] Client handlers listen for updates to central chat history, update their client on change
  * [X] Recieves new chat messages from assigned client, adds them to central chat history
