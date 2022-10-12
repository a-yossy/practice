<!DOCTYPE html>
<html>
<head>
<meta charset="utf-8">
<title>ToDoリスト</title>
</head>
<body>
  <form action="/todolist/public/create.php" method="post">
    <h2>作成ぺージ</h2>
    <p><a href="/todolist/public/index.php">ToDo一覧</a></p>
    <div>
      <p>予定タイトル</p>
      <input type="text" name="title">
    </div>
    <div>
      <p>優先度</p>
      <label><input type="radio" name="priority" value="高">高</label>
      <label><input type="radio" name="priority" value="中">中</label>
      <label><input type="radio" name="priority" value="低">低</label>
    </div>
    <div>
      <p>予定種別</p>
      <select name="type">
        <option value="">-選択-</option>
        <option value="外出">外出</option>
        <option value="家事">家事</option>
        <option value="仕事">仕事</option>
        <option value="勉強">勉強</option>
      </select>
    </div>
    <div>
      <p>予定詳細</p>
      <textarea name="body" rows="5"></textarea>
    </div>

    <div>
      <p><input type="submit" value="作成"></p>
    </div>
  </form>
</body>
</html>
