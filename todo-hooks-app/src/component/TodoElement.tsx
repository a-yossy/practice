import { TodoListElement } from "../domain/entity";
import Button from "./Button";

import * as React from "react";

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

export default TodoListNode;