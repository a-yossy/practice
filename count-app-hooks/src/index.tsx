import * as React from "react";
import ReactDOM from "react-dom";
const { useState } = React;


const CountApp: React.FC = () => {
  const [count, setCount] = useState(0);
  return (
    <>
      <h1>Count App</h1>
      <div>{count}</div>
      <button onClick={() => setCount(count + 1)}>+</button>
      <button onClick={() => setCount(count - 1)}>-</button>
    </>
  )
}

ReactDOM.render(
  <CountApp />,
  document.getElementById('root')
)