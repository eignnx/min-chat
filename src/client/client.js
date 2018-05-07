const KNOWN_SERVER_ADDRESS = "ws:10.0.0.241:9090";

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


$(() => {
    $("#loginModal").modal("show");
    $("#my-login-button").click(function() {
        let username = $("#username-field")[0].value;
        $("#loginModal").modal("hide");
        
        let websocket = new WebSocket(KNOWN_SERVER_ADDRESS);

        websocket.onmessage = (event) => {
            let msg = ChatMessage.fromJSON(JSON.parse(event.data));
            $("#chat-history-window")[0].innerHTML += msg.toHTML();
        };

        // let newUser = { "username": username };
        // websocket.send(JSON.stringify(newUser));

        $("#send-message-button").click(function() {
            console.log("Sending...");
            let textBox = $("#chat-text-box")[0];
            let msg = new ChatMessage(username, textBox.value, Date.now());
            console.log(msg);
            msg.transmit(websocket);
            textBox.value = "";
        });

    });
});
