# min-chat
A minimal (or at least very small) chat client/server implementation written in Rust.

## The Plan
* [ ] Javascript clients run in browser
  * [ ] Listen for new chat messages sent from server, updates client's chat history
  * [ ] Allow users to type messages while new ones are being recieved simultaneously
  * [ ] Send new chat messages to the server over the WebSocket
* [ ] Rust-lang server
  * [ ] Listens for new client connnects, spawns thread to handle each new client
  * [ ] Client handlers listen for updates to central chat history, update their client on change
  * [ ] Recieves new chat messages from assigned client, adds them to central chat history
