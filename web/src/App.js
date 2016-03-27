import React, { Component } from 'react';

import Connection from './connection';
import Game from './Game';

export default class App extends Component {
    constructor () {
        super();
        this.state = {
            // connection: new Connection((ws) => {
            //     this.setState({ connected: true });
            // }, (msg) => {
            //     msg = JSON.parse(msg);
            //     this.setState({ game: msg });
            // }),
            connected: true,
            game: {
                "my_hand": [21, 30, 11, 4, 28, 35, 8, 19],
                "my_taken": [],
                "his_hand_count": 8,
                "his_taken": [],
                "table": [7, 41, 14, 9, 16, 39, 18, 0],
                "deck_left": 24
            }
        }
    }
    sendMessage (content) {
        this.state.connection.send(content);
    }
    render() {
        console.log(this.state);
        return (
            <div>
                {(
                    !this.state.connected ?
                        <p>Not connected</p>
                    :
                        this.state.game === null ?
                            <p>Connected, waiting</p>
                        :
                            <Game {...this.state.game} />
                )}
            </div>
        );
    }
}
