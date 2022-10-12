<?php
  require_once('../classes/todo.php');
  require_once('../lib/security.php');

  $todo = new Todo();
  $todoData = $todo->getAll();
?>

<!DOCTYPE html>
<html>
<head>
<meta charset="utf-8">
<title>ToDoリスト</title>
</head>
<body>
  <h2>一覧ページ</h2>
  <p><a href='/todolist/public/new.php'>新規作成</a></p>
  <?php foreach($todoData as $todo): ?>
    <ul>
      <li>
        <a href="/todolist/public/show.php?id=<?php echo $todo['id'] ?>"><?php echo h($todo['title']) ?></a>
      </li>
      <a href="/todolist/public/edit.php?id=<?php echo $todo['id'] ?>">編集</a>
      <a href="/todolist/public/delete.php?id=<?php echo $todo['id'] ?>">削除</a>
    </ul>
  <?php endforeach ?>
</body>
</html>
