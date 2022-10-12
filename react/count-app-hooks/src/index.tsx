import Button from "./Button";

import * as React from "react";
import ReactDOM from "react-dom";
const { useState } = React;

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