import { TodoListElement } from "../domain/entity";
import Button from "./Button";
import TextInput from "./TextInput";

import * as React from "react";

type AddTodoProps = {
  onChange: (e: string | number) => void,
  value: string | number,
  id: number,
  onAdd: (todoElement: TodoListElement) => void,
}

const AddTodo: React.FC<AddTodoProps> = ({ onChange, value, id, onAdd }) => {
  const add = (): void => {
    const todoElement: TodoListElement = {
      content: value,
      id: id + 1,
    }
    onAdd(todoElement)
  }

  return(
    <>
      <TextInput
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

export default AddTodo;