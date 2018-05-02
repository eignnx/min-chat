
const KNOWN_SERVER_ADDRESS = "ws:127.0.0.1:9090";
const PROTOCOL = "MinChatProtocol";

let websocket = new WebSocket(KNOWN_SERVER_ADDRESS);
let username = "TestUser123";

class ChatMessage {
    constructor(sender, body, timestamp) {
        this.sender = sender;
        this.body = body;
        this.timestamp = timestamp;
    }

    static fromJSON(obj) {
        return new ChatMessage(obj.sender, obj.body, obj.timestamp);
    }

    transmit(websocket) {
        websocket.send(JSON.stringify(this));
    }

    toHTML() {
        return `
            <div class="row my-3">
                <div class="col"></div>
                <div class="col-10">
                    <div class="card">
                        <div class="card-header">
                            <div class="row">
                                <div class="col"><strong>${this.sender}</strong></div>
                                <div class="col text-muted text-right">${new Date(this.timestamp).toLocaleString()}</div>
                            </div>
                        </div>
                        <div class="card-body">
                            ${this.body}
                        </div>
                    </div>
                </div>
                <div class="col"></div>
            </div>
        `;
    }
}

function sendTextboxChatMessage() {
    let textBox = document.getElementById("chat-text-box");
    let msg = new ChatMessage(username, textBox.value, Date.now());
    msg.transmit(websocket);
    textBox.value = "";
}

websocket.onmessage = (event) => {
    let msg = ChatMessage.fromJSON(JSON.parse(event.data));
    let window = document.getElementById("chat-history-window");
    window.innerHTML += msg.toHTML();
}