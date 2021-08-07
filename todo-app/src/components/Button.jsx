import React from "react";

class Button extends React.Component {
  render() {
    return (
      <button onClick={this.props.onClick}>{this.props.buttonText}</button>
    )
  }
}

export default Button;