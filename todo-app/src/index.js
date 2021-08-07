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

  onAdd() {
    const todoElement = {
      content: this.props.value,
      id: this.props.todoList.length + 1,
    }
    this.props.onAdd(todoElement)
  }

  render() {
    return(
      <div>
        <input
          type="text"
          value={this.props.value}
          onChange={e => this.onChange(e)}
        />
        <button onClick={() => this.onAdd()}>追加</button>
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

  onEdit() {
    let todoList = this.props.todoList.concat()
    todoList.map((element) => {
      if (element.id === this.props.editTodoId) {
        element.content = this.props.value
      }
    })
    this.props.onEdit(todoList)
  }

  onCancel() {
    this.props.onCancel()
  }

  render() {
    return(
      <div>
        <input
          type="text"
          value={this.props.value}
          onChange={e => this.onChange(e)}
        />
        <button onClick={() => this.onEdit()}>更新</button>
        <button onClick={() => this.onCancel()}>キャンセル</button>
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

  handleChange(key_value) {
    this.setState(key_value)
  }

  handleAdd(todoElement) {
    this.setState({
      todoList: this.state.todoList.concat(todoElement),
      value: "",
    })
  }

  handleEdit(todoList) {    
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
          : <EditTodo
              {...this.state}
              onChange={e => this.handleChange(e)}
              onEdit={todoList => this.handleEdit(todoList)}
              onCancel={() => this.handleCancel()}
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