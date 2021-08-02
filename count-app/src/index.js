import React from 'react';
import ReactDOM from 'react-dom';

class Button extends React.Component {
  render() {
    return (
      <button onClick={this.props.handleClick}>{this.props.buttonText}</button>
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
        <Button
          buttonText='+'
          handleClick={() => this.setState({
            count: this.state.count + 1,
          })}
        />
        <Button
          buttonText='-'
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
