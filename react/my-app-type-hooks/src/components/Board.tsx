import React from "react";
import { ISquare } from "../domain/entity";
import Square from "./Square";

interface BoardProps {
  squares: ISquare[];
  onClick: (i: number) => void;
}

const Board: React.FC<BoardProps> = ({ squares, onClick }) => {
  return (
    <div className="board">
      {Array<number>(9)
        .fill(0)
        .map((_, i) => (
          <Square key={i} value={squares[i]} onClick={() => onClick(i)}/>
        ))}
    </div>
  );
};

export default Board;