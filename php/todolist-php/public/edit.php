<?php
  require_once('../classes/todo.php');
  require_once('../lib/security.php');

  $id = $_GET['id'];
  $todo = new Todo();
  $result = $todo->getById($id);

  $title = $result['title'];
  $priority = $result['priority'];
  $type = $result['type'];
  $body = $result['body'];
?>

<!DOCTYPE html>
<html>
<head>
<meta charset="utf-8">
<title>ToDoリスト</title>
</head>
<body>
  <form action="/todolist/public/update.php" method="post">
    <h2>編集ぺージ</h2>
    <p><a href="/todolist/public/index.php">ToDo一覧</a></p>
    <input type="hidden" name="id" value="<?php echo $id ?>">
    <div>
      <p>予定タイトル</p>
      <input type="text" name="title" value="<?php echo h($title) ?>">
    </div>
    <div>
      <p>優先度</p>
      <label><input type="radio" name="priority" value="高" <?php if ($priority === '高') echo "checked" ?>>高</label>
      <label><input type="radio" name="priority" value="中" <?php if ($priority === '中') echo "checked" ?>>中</label>
      <label><input type="radio" name="priority" value="低" <?php if ($priority === '低') echo "checked" ?>>低</label>
    </div>
    <div>
      <p>予定種別</p>
      <select name="type">
        <option value="">-選択-</option>
        <option value="外出" <?php if ($type === '外出') echo "selected" ?>>外出</option>
        <option value="家事" <?php if ($type === '家事') echo "selected" ?>>家事</option>
        <option value="仕事" <?php if ($type === '仕事') echo "selected" ?>>仕事</option>
        <option value="勉強" <?php if ($type === '勉強') echo "selected" ?>>勉強</option>
      </select>
    </div>
    <div>
      <p>予定詳細</p>
      <textarea name="body" rows="5"><?php echo h($body) ?></textarea>
    </div>

    <div>
      <p><input type="submit" value="更新"></p>
    </div>
  </form>
</body>
</html>
