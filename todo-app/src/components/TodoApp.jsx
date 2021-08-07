import React from "react";
import TodoElement from "./TodoElement";
import AddTodo from "./AddTodo";
import UpdateTodo from "./UpdateTodo";

class TodoApp extends React.Component {
  constructor() {
    super()
    this.state={
      todoList: [],
      value: "",
      isEditMode: false,
      editTodoId: 0,
    }
  }

  handleChange(key_value) {
    this.setState(key_value)
  }

  handleAdd(todoElement) {
    this.setState({
      todoList: this.state.todoList.concat(todoElement),
      value: "",
    })
  }

  handleUpdate(todoList) {    
    this.setState({todoList: todoList, value: '', isEditMode: false, editTodoId: 0})
  }

  handleCancel() {
    this.setState({value: '', isEditMode: false, editTodoId: 0})
  }

  handleDelete(id) {
    let todoList = this.state.todoList.concat()
    let index = 0
    todoList.map((element, idx) => {
      if (element.id === id) {
        index = idx
      }
    })
    todoList.splice(index, 1)
    this.setState({todoList: todoList})
  }

  render() {
    const todoListNode = this.state.todoList.map(element => {
      return (
        <TodoElement
          element={element}
          onDelete={() => this.handleDelete()}
          onEdit={() => this.setState({isEditMode: true, value: element.content, editTodoId: element.id,})}
        />
      )
    })

    return (
      <div>
        <h1>TODO App</h1>
        {!this.state.isEditMode
          ? <>
              <AddTodo
                {...this.state}
                onChange={e => this.handleChange(e)}
                onAdd={todoElement => this.handleAdd(todoElement)}
              />
              <ul>
                {todoListNode}
              </ul>
            </>
          : <UpdateTodo
              {...this.state}
              onChange={e => this.handleChange(e)}
              onUpdate={todoList => this.handleUpdate(todoList)}
              onCancel={() => this.handleCancel()}
            />
        }
      </div>
    );
  }
}

export default TodoApp;