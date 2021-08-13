import { TodoListElement } from "domain/entity";
import Button from "component/Button";
import TextInput from "component/TextInput";

import * as React from "react";

type AddTodoProps = {
  onChange: (e: string | number) => void,
  value: string | number,
  id: number,
  onAdd: (todoElement: TodoListElement) => void,
}

const AddTodo: React.FC<AddTodoProps> = ({ onChange, value, id, onAdd }) => {
  const addElement = (): void => {
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
        onClick={addElement}
      />
    </>
  )
}

export default AddTodo;