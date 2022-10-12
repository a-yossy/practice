<?php
  require_once('../classes/todo.php');

  $todoParams = $_POST;
  $todo = new Todo();
  $todo->todoValidate($todoParams);
  $todo->todoCreate($todoParams);
?>

<p><a href="/todolist/public">TOPページ</a></p>
