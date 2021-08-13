import * as React from "react";

type TextInputProps = {
  value: string | number,
  onChange: (e: string | number) => void,
}

const TextInput: React.FC<TextInputProps> = ({ value, onChange }) => {
  return(
    <input
      type="text"
      value={value}
      onChange={(e) => onChange(e.target.value)}
    />
  )
}

export default TextInput;