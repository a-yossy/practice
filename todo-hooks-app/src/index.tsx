import * as React from "react";
import ReactDOM from "react-dom";
const { useState } = React;

type TodoListNodeProps = {
  todoList: TodoListElement[],
  onDelete: (id: number) => void,
}

const TodoListNode: React.FC<TodoListNodeProps> = ({ todoList, onDelete }) => {
  return (
    <>
      {todoList.map((todoElement :TodoListElement) => {
        return(
          <li key={todoElement.id}>
            {todoElement.content}
            <button onClick={() => onDelete(todoElement.id)}>削除</button>
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
      if (element.id == id) {
        index = idx
      }
    })
    newTodoList.splice(index, 1)
    setTodoList(newTodoList)
  }
  
  return (
    <div>
      <h1>TODO App</h1>
        <input
          type="text"
          value={value}
          onChange={e => onChange(e)}
        />
      <button onClick={add}>追加</button>
      <ul>
        <TodoListNode
          todoList={todoList}
          onDelete={handleDelete}
        />
      </ul>
    </div>
  );
}

ReactDOM.render(
  <TodoApp />,
  document.getElementById('root')
);