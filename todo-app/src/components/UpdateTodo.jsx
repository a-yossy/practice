import React from "react";
import TextInput from "./TextInput";
import Button from "./Button";

class UpdateTodo extends React.Component {
  onChange(e) {
    this.props.onChange({
      value: e.target.value,
    })
  }

  onUpdate() {
    let todoList = this.props.todoList.concat()
    todoList.map((element) => {
      if (element.id === this.props.editTodoId) {
        element.content = this.props.value
      }
    })
    this.props.onUpdate(todoList)
  }

  onCancel() {
    this.props.onCancel()
  }

  render() {
    return(
      <div>
        <TextInput
          value={this.props.value}
          onChange={e => this.onChange(e)}
        />
        <Button
          buttonText='更新'
          onClick={() => this.onUpdate()}
        />
        <Button
          buttonText='キャンセル'
          onClick={() => this.onCancel()}
        />
      </div>
    )
  }
}

export default UpdateTodo;