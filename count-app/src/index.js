import React from 'react';
import ReactDOM from 'react-dom';

class PlusButton extends React.Component {
  render() {
    return (
      <button onClick={this.props.handleClick}>+</button>
    )
  }
}

class MinusButton extends React.Component {
  render() {
    return (
      <button onClick={this.props.handleClick}>
        -
      </button>
    )
  }
}

class CountApp extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      count: 0,
    };
  }

  render() {
    return (
      <>
        <h1>Count App</h1>
        <div>{this.state.count}</div>
        <PlusButton
          handleClick={() => this.setState({
            count: this.state.count + 1,
          })}
        />
        <MinusButton
          handleClick={() => this.setState({
            count: this.state.count - 1,
          })} 
        />
      </>
    )
  }
}

// ========================================

ReactDOM.render(
  <CountApp />,
document.getElementById('root')
);
