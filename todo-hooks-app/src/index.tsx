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
      {todoList.map((todoElement: TodoListElement) => {
        return(
          <li key={todoElement.id}>
            {todoElement.content}
            <Button
              buttonText="削除"
              onClick={() => onDelete(todoElement.id)}
            />
            <Button
              buttonText="編集"
              onClick={() => onEdit(todoElement)}
            />
          </li>
        );
      })}
    </>
  );
}

type ButtonProps = {
  buttonText: string,
  onClick: () => void,
}

const Button: React.FC<ButtonProps> = ({ buttonText, onClick }) => {
  return(
    <button onClick={onClick}>{buttonText}</button>
  )
}

type AddTodoProps = {
  onChange: (e: React.ChangeEvent<HTMLInputElement>) => void,
  value: string | number,
  id: number,
  onAdd: (todoElement: TodoListElement) => void,
}

const AddTodo: React.FC<AddTodoProps> = ({ onChange, value, id, onAdd }) => {
  const add = (): void => {
    const todoElement = {
      content: value,
      id: id + 1,
    }
    onAdd(todoElement)
  }

  return(
    <>
      <input
        type="text"
        value={value}
        onChange={onChange}
      />
      <Button
        buttonText="追加"
        onClick={add}
      />
    </>
  )
}

type UpdateTodoProps = {
  onChange: (e: React.ChangeEvent<HTMLInputElement>) => void,
  value: string | number,
  onUpdate: () => void,
  onCancel: () => void,
}

const UpdateTodo: React.FC<UpdateTodoProps> = ({ onChange, value, onUpdate, onCancel }) => {
  return(
    <>
      <input
        type="text"
        value={value}
        onChange={onChange}
      />
      <Button
        buttonText="更新"
        onClick={onUpdate}
      />
      <Button
        buttonText="キャンセル"
        onClick={onCancel}
      />
    </>
  )
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

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>): void => {
    setValue(e.target.value)
  }

  const handleAdd = (todoElement: TodoListElement): void => {
    setTodoList([
      ...todoList,
      todoElement
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
        ? <UpdateTodo 
            onChange={handleChange}
            value={value}
            onUpdate={handleUpdate}
            onCancel={handleCancel}
          />
        : <>
            <AddTodo
              onChange={handleChange}
              value={value}
              id={id}
              onAdd={todoElement => handleAdd(todoElement)}
            />
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