import * as React from "react";
import ReactDOM from "react-dom";
const { useState } = React;

type TodoListNodeProps = {
  todoList: TodoListElement[],
  onDelete: (id: number) => void,
  onEdit: (todoElement: TodoListElement) => void,
}

const TodoListNode: React.FC<TodoListNodeProps> = ({ todoList, onDelete, onEdit }) => {
  return (
    <>
      {todoList.map((todoElement :TodoListElement) => {
        return(
          <li key={todoElement.id}>
            {todoElement.content}
            <button onClick={() => onDelete(todoElement.id)}>削除</button>
            <button onClick={() => onEdit(todoElement)}>編集</button>
          </li>
        );
      })}
    </>
  );
}

type TodoListElement = {
  content: string | number,
  id: number,
}

const TodoApp: React.FC = () => {
  const [todoList, setTodoList] = useState<TodoListElement[]>([]);
  const [value, setValue] = useState<string | number>("");
  const [id, setId] = useState<number>(0);
  const [isEditMode, setIsEditMode] = useState<boolean>(false);
  const [editTodoId, setEditTodoId] = useState<number>(0);
  
  const onChange = (e: React.ChangeEvent<HTMLInputElement>): void => {
    setValue(e.target.value)
  }

  const add = (): void => {
    const newTodo: TodoListElement = { 
      content: value, 
      id: id + 1,
    }
    setTodoList([
      ...todoList,
      newTodo
    ]);
    setId(id + 1);
    setValue("");
  };

  const handleDelete = (id: number): void => {
    let newTodoList: TodoListElement[] = todoList.concat()
    let index: number = 0
    newTodoList.map((element, idx) => {
      if (element.id === id) {
        index = idx
      }
    })
    newTodoList.splice(index, 1)
    setTodoList(newTodoList)
  }

  const handleEdit = (todoElement: TodoListElement): void => {
    setIsEditMode(true);
    setValue(todoElement.content);
    setEditTodoId(todoElement.id);
  }

  const handleUpdate = (): void => {
    let newTodolist: TodoListElement[] = todoList.concat()
    newTodolist.map((todoElement) => {
      if (todoElement.id === editTodoId) {
        todoElement.content = value
      }
    })
    setTodoList(newTodolist);
    setValue("");
    setIsEditMode(false);
    setEditTodoId(0);
  }

  const handleCancel = (): void => {
    setValue("");
    setIsEditMode(false);
    setEditTodoId(0);
  }
  
  return (
    <div>
      <h1>TODO App</h1>
      {isEditMode 
        ? <>
            <input
              type="text"
              value={value}
              onChange={onChange}
            />
            <button onClick={handleUpdate}>更新</button>
            <button onClick={handleCancel}>キャンセル</button>
          </>
        : <>
            <input
              type="text"
              value={value}
              onChange={onChange}
            />
            <button onClick={add}>追加</button>
            <ul>
              <TodoListNode
                todoList={todoList}
                onDelete={handleDelete}
                onEdit={handleEdit}
              />
            </ul>
          </>
      }
    </div>
  );
}

ReactDOM.render(
  <TodoApp />,
  document.getElementById('root')
);