<?php
  require_once('../classes/todo.php');

  $id = $_GET['id'];
  $todo = new Todo();
  $result = $todo->delete($id);
?>

<p><a href="/todolist/public">TOPページ</a></p>
