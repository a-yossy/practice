import * as React from "react";
import ReactDOM from "react-dom";
const { useState } = React;

type TodoListElement = {
  content: string | number,
  id: number,
}

const TodoApp: React.FC = () => {
  const [todoList, setTodoList] = useState<TodoListElement[]>([]);
  const [value, setValue] = useState<string | number>("");
  const [id, setIdNumber] = useState<number>(0);
  
  const onChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setValue(e.target.value)
  }

  const add: () => void = () => {
    const newTodo = { 
      content: value, 
      id: id + 1,
    }
    setTodoList([
      ...todoList,
      newTodo
    ]);
    setIdNumber(id + 1);
    setValue("");
  };

  const todoListNode = todoList.map(element => {
    return(
      <li key={element.id}>
        {element.content}
      </li>
    )
  })
  
  return (
    <div>
      <h1>TODO App</h1>
        <input
          type="text"
          value={value}
          onChange={e => onChange(e)}
        />
      <button onClick={() => add()}>追加</button>
      <ul>
        {todoListNode}
      </ul>
    </div>
  );
}

ReactDOM.render(
  <TodoApp />,
  document.getElementById('root')
);