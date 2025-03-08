CREATE TABLE
  todo (
    id TEXT PRIMARY KEY NOT NULL,
    text TEXT NOT NULL,
    completed BOOLEAN NOT NULL DEFAULT false
  );
