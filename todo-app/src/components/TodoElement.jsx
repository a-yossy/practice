import React from "react";
import Button from "./Button";

class TodoElement extends React.Component {
  onDelete() {
    this.props.onDelete(this.props.element.id)
  }

  render() {
    return(
      <li key={this.props.element.id}>
        <span>{this.props.element.content}</span>
        <Button
          buttonText='編集'
          onClick={() => this.props.onEdit()}
        />
        <Button
          buttonText='削除'
          onClick={() => this.onDelete()}
        />
      </li>
    )
  }
}

export default TodoElement;