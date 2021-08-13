import { TodoListElement } from "domain/entity";
import Button from "component/Button";
import TextInput from "component/TextInput";

import * as React from "react";

type UpdateTodoProps = {
  onChange: (value: string | number) => void,
  value: string | number,
  todoList: TodoListElement[],
  editTodoId: number,
  onUpdate: (newTodoList: TodoListElement[]) => void,
  onCancel: () => void,
}

const UpdateTodo: React.FC<UpdateTodoProps> = ({ onChange, value, todoList, editTodoId, onUpdate, onCancel }) => {
  const updateList = (): void => {
    let newTodoList: TodoListElement[] = todoList.concat()
    newTodoList.map((todoElement) => {
      if (todoElement.id === editTodoId) {
        todoElement.content = value
      }
    })
    onUpdate(newTodoList)
  }
  
  return(
    <>
      <TextInput
        value={value}
        onChange={onChange}
      />
      <Button
        buttonText="更新"
        onClick={updateList}
      />
      <Button
        buttonText="キャンセル"
        onClick={onCancel}
      />
    </>
  )
}

export default UpdateTodo;