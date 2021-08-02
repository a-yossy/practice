import React from 'react';
import ReactDOM from 'react-dom';
import './index.css';

class Square extends React.Component {
    render() {
      return (
        <button 
          className="square"
          onClick={() => this.props.onClick()}  
        >
          {this.props.value}
        </button>
      );
    }
  }
  
class Count extends React.Component {
constructor(props) {
    super(props);
    this.state = {
        value: ['+', '-'],
        count: 0, 
    };
}

handleClick(i) {
    let count = this.state.count;
    if (i) {
    count -= 1;
    } else {
    count += 1;
    }
    this.setState({
    count: count,
    })
}

renderSquare(i) {
    return (
    <Square 
        value={this.state.value[i]}
        onClick={() => this.handleClick(i)}
    />
    );
}

render() {
    const status = 'カウント値： ' + this.state.count;

    return (
    <div>
        <div className="status">{status}</div>
        <div className="board-row">
        {this.renderSquare(0)}
        {this.renderSquare(1)}
        </div>
    </div>
    );
}
}

class Game extends React.Component {
render() {
    return (
    <div className="game">
        <div className="game-board">
        <Count />
        </div>
        <div className="game-info">
        <div>{/* status */}</div>
        <ol>{/* TODO */}</ol>
        </div>
    </div>
    );
}
}

// ========================================

ReactDOM.render(
<Game />,
document.getElementById('root')
);
  
  