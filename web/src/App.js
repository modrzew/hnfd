import React, { Component } from 'react';

import Connection from './connection';

export default class App extends Component {
    constructor () {
        super();
        this.state = {
            connection: new Connection((ws) => {
                console.log('Connected!');
            }, (msg) => {
                msg = JSON.parse(msg);
                console.log(msg);
            }),
        }
    }
    sendMessage (content) {
        this.state.connection.send(content);
    }
    render() {
        return (
            <div>
                {(
                    !this.state.connected ?
                        <p>Not connected</p>
                    :
                        <p>Connected!</p>
                )}
            </div>
        );
    }
}
