import React from "react";
import { Histories } from "../domain/entity";

interface MoveProps {
  histories: Histories;
  jumpTo: (move: number) => void;
}

const Moves: React.FC<MoveProps> = ({ histories, jumpTo }) => (
  <ol>
    {histories.map((_, move) => {
      const desc = move ? `Go to move #${move}` : "Go to game start";
      return (
        <li key={move}>
          <button onClick={() => jumpTo(move)}>{desc}</button>
        </li>
      );
    })}
  </ol>
);

export default Moves;