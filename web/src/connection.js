export default function Connection (onReady, onMessage) {
    var that = this;
    var uri = "ws://" + window.location.hostname + ":3012/";

    this.ws = new WebSocket(uri);

    this.ws.onopen = function() {
        console.log('CONN: Connection to ' + uri + ' opened');
        onReady(that.ws);
    }
    this.ws.onmessage = function(evt) {
        var msg = evt.data;
        console.log('CONN: Message received: ' + msg);
        onMessage(JSON.parse(msg));
    }
    this.send = function(msg) {
        that.ws.send(msg);
    }
}
