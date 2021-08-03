import React from 'react';
import ReactDOM from 'react-dom';

class TodoElement extends React.Component {
  onDelete() {
    this.props.onDelete(this.props.element.id)
  }

  onEdit() {
    this.props.onEdit(this.props.element.id)
  }

  render() {
    return(
      <li>
        <span>{this.props.element.content}</span>
        <button onClick={() => this.onDelete()}>削除</button>
        <button onClick={() => this.onEdit()}>編集</button>
      </li>
    )
  }
}


class AddTodo extends React.Component {
  onChange(e) {
    this.props.onChange({
      value: e.target.value,
    })
  }

  add() {
    const todoElement = {
      content: this.props.value,
      id: this.props.todoList.length + 1,
    }
    this.props.add(todoElement)
  }

  render() {
    return(
      <div>
        <input
          type="text"
          value={this.props.value}
          onChange={e => this.onChange(e)}
        />
        <button onClick={() => this.add()}>追加</button>
      </div>
    )
  }
}


class TodoApp extends React.Component {
  constructor() {
    super()
    this.state={
      todoList: [],
      value: "",
    }
  }

  onChange(key_value) {
    this.setState(key_value)
  }

  add(todoElement) {
    this.setState({
      todoList: this.state.todoList.concat(todoElement),
      value: "",
    })
  }

  edit(id) {
    const edittodoElement = {
      content: this.state.value,
      id: id,
    }
    let todoList = this.state.todoList.concat()
    todoList.splice(id - 1, 1)
    todoList.splice(id - 1, 0, edittodoElement)
    this.setState({
      todoList: todoList.concat(),
      value: "",
    })
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
        <li key={element.id}>
          {element.content}
          <button onClick={() => this.edit(element.id)}>編集</button>
        </li>
      )
    })

    return (
      <div>
        <h1>TODO App</h1>
        <AddTodo
          {...this.state}
          onChange={e => this.onChange(e)}
          add={todoElement => this.add(todoElement)}
        />
        <ul>
          {todoListNode}
        </ul>
      </div>
    );
  }
}

ReactDOM.render(
  <TodoApp />,
  document.getElementById('root')
);