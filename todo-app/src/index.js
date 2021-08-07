import React from 'react';
import ReactDOM from 'react-dom';

class TodoElement extends React.Component {
  onDelete() {
    this.props.onDelete(this.props.element.id)
  }

  render() {
    return(
      <li key={this.props.element.id}>
        <span>{this.props.element.content}</span>
        <button onClick={() => this.onDelete()}>削除</button>
        <button onClick={() => this.props.onEdit()}>編集</button>
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

class EditTodo extends React.Component {
  onChange(e) {
    this.props.onChange({
      value: e.target.value,
    })
  }

  edit() {
    let todoList = this.props.todoList.concat()
    todoList.map((element) => {
      if (element.id === this.props.editTodoId) {
        element.content = this.props.value
      }
    })
    this.props.edit(todoList)
  }

  cancel() {
    let todoList = this.props.todoList.concat()
    this.props.edit(todoList)
  }

  render() {
    return(
      <div>
        <input
          type="text"
          value={this.props.value}
          onChange={e => this.onChange(e)}
        />
        <button onClick={() => this.edit()}>更新</button>
        <button onClick={() => this.cancel()}>キャンセル</button>
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
      isEditMode: false,
      editTodoId: 0,
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

  edit(todoList) {    
    this.setState({todoList: todoList, value: '', isEditMode: false, editTodoId: 0})
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
                onChange={e => this.onChange(e)}
                add={todoElement => this.add(todoElement)}
              />
              <ul>
                {todoListNode}
              </ul>
            </>
          : <EditTodo
              {...this.state}
              onChange={e => this.onChange(e)}
              edit={todoList => this.edit(todoList)}
            />
        }
      </div>
    );
  }
}

ReactDOM.render(
  <TodoApp />,
  document.getElementById('root')
);