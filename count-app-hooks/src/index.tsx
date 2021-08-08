import * as React from "react";
import ReactDOM from "react-dom";
const { useState } = React;

type ButtonProps = {
  buttonText: string,
  onClick: () => void,
}

const Button: React.FC<ButtonProps> = ({ buttonText, onClick }) => {
  return (
    <button onClick={() => onClick()}>{buttonText}</button>
  )
}

const CountApp: React.FC = () => {
  const [count, setCount] = useState(0);

  return (
    <>
      <h1>Count App</h1>
      <div>{count}</div>
      <Button
        buttonText="+"
        onClick={() => setCount(count + 1)}
      />
      <Button
        buttonText="-"
        onClick={() => setCount(count - 1)}
      />
    </>
  )
}

ReactDOM.render(
  <CountApp />,
  document.getElementById('root')
)