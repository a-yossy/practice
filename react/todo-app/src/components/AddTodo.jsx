import React from "react";
import TextInput from "./TextInput";
import Button from "./Button";

class AddTodo extends React.Component {
  onChange(e) {
    this.props.onChange({
      value: e.target.value,
    })
  }

  onAdd() {
    const todoElement = {
      content: this.props.value,
      id: this.props.id + 1,
    }
    this.props.onAdd(todoElement)
  }

  render() {
    return(
      <div>
        <TextInput
          value={this.props.value}
          onChange={e => this.onChange(e)}
        />
        <Button
          buttonText='追加'
          onClick={() => this.onAdd()}
        />
      </div>
    )
  }
}

export default AddTodo;