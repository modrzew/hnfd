import React, { Component } from 'react';


class Card extends Component {
    render () {
        return <li>{this.props.id}</li>;
    }
}


class Hand extends Component {
    render () {
        var cards = [];
        if (typeof this.props.cardsCount !== 'undefined') {
            for(var i=0; i<this.props.cardsCount; i++) {
                cards.push(<Card key={i} id={-1} />);
            }
        } else {
            cards = this.props.cards.map((card) => <Card key={card} id={card} />);
        }
        return <ul>{cards}</ul>;
    }
}


class Table extends Component {
    render () {
        return (
            <ul>
                {this.props.cards.map((card) => <Card key={card} id={card} />)}
            </ul>
        );
    }
}


export default class Game extends Component {
    send () {
        this.props.sendMessage('{"from":1,"to":3}');
    }

    render () {
        return (
            <div onClick={this.send.bind(this)}>
                <Hand cardsCount={this.props.his_hand_count} />
                <Table cards={this.props.table} />
                <Hand cards={this.props.my_hand} />
            </div>
        );
    }
}
