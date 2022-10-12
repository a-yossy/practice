import { ISquare, Histories } from "./entity";

export function calculateWinner(squares: ISquare[]) {
  const lines = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 4, 6],
  ];
  for (let i = 0; i < lines.length; i++) {
    const [a, b, c] = lines[i];
    if (squares[a] && squares[a] === squares[b] && squares[a] === squares[c]) {
      return squares[a];
    }
  }
  return null;
}

export const getStatus = (winner: ISquare, xIsNext: boolean) => {
  if (winner) {
    return `Winner: ${winner}`;
  } else {
    const nextPlayer = xIsNext ? "X" : "O";
    return `Next player: ${nextPlayer}`;
  }
};

export const createNewSquares = (
  squares: ISquare[],
  xIsNext: boolean,
  i: number
) =>
  squares.map((square, index) => {
    if (i === index) {
      return xIsNext ? "X" : "O";
    }
    
    return square;
  });





