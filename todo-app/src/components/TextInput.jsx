import React from "react";

class TextInput extends React.Component {
  render() {
    return (
      <input
        type="text"
        value={this.props.value}
        onChange={this.props.onChange}
      />
    )
  }
}

export default TextInput;