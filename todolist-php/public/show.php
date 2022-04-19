<?php
  require_once('../classes/todo.php');
  require_once('../lib/security.php');

  $id = $_GET['id'];
  $todo = new Todo();
  $result = $todo->getById($id);
?>

<!DOCTYPE html>
<html>
<head>
<meta charset="utf-8">
<title>ToDoリスト</title>
</head>
<body>
  <h2>詳細ページ</h2>
  <p><a href="/todolist/public/index.php">ToDo一覧</a></p>
  <div>
    <h3>予定タイトル</h3>
    <?php echo h($result['title']) ?>
  </div>
  <div>
    <h3>優先度</h3>
    <?php echo h($result["priority"]) ?>
  </div>
  <div>
    <h3>予定種別</h3>
    <?php echo h($result["type"]) ?>
  </div>
  <div>
    <h3>予定詳細</h3>
    <?php echo nl2br(h($result["body"])) ?>
  </div>
</body>
</html>
