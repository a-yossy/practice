import { TodoListElement } from "domain/entity";
import Button from "component/Button";

import * as React from "react";

type TodoListNodeProps = {
  todoList: TodoListElement[],
  onDelete: (newTodoList: TodoListElement[]) => void,
  onEdit: (todoElement: TodoListElement) => void,
}

const TodoListNode: React.FC<TodoListNodeProps> = ({ todoList, onDelete, onEdit }) => {
  const deleteTodo = (id: number): void => {
    let newTodoList: TodoListElement[] = todoList.concat()
    let index: number = 0
    newTodoList.map((todoElement, idx) => {
      if (todoElement.id === id) {
        index = idx
      }
    })
    newTodoList.splice(index, 1)
    onDelete(newTodoList)
  }

  return (
    <>
      {todoList.map((todoElement: TodoListElement) => {
        return(
          <li key={todoElement.id}>
            {todoElement.content}
            <Button
              buttonText="削除"
              onClick={() => deleteTodo(todoElement.id)}
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

export default TodoListNode;