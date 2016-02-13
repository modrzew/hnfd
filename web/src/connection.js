export default function Connection (ready, recv) {
    var that = this;
    var uri = "ws://" + window.location.hostname + ":9000/";

    this.ws = new WebSocket(uri);

    this.ws.onopen = function() {
        console.log('CONN: Connection to ' + uri + ' opened');
        ready(that.ws);
    }
    this.ws.onmessage = function(evt) {
        var msg = evt.data;
        console.log('CONN: Message received: ' + msg);
        recv(msg);
    }
    this.send = function(msg) {
        that.ws.send(msg);
    }
}

/**
 * HOW TO USE:
 *
 * function connectionReadyExample(ws) {
 *     console.log('READY CALLBACK: Sending Hello!')
 *     ws.send('Hello!');
 * }
 *
 * function msgReceivedExample(msg) {
 *     console.log('RECEIVED CALLBACK: Message received: ' + msg );
 * }
 *
 * function timeoutExample() {
 *     console.log('TIMEOUT: Sending: 1000!');
 *     conn.send('1000!');
 * }
 *
 * var conn = new Connection(connectionReadyExample, msgReceivedExample);
 * setTimeout(timeoutExample, 1000);
 *
 */
