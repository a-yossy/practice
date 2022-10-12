<?php
  require_once('dbc.php');
  $files = getAllFile();
?>

<!DOCTYPE html>
<html>
<head>
  <meta charset="utf-8">
  <title>アップロードフォーム</title>
</head>
<body>
  <form enctype="multipart/form-data" action="/upload/public/upload.php" method="post">
    <div>
      <input type="hidden" name="MAX_FILE_SIZE" value="1048576">
      <input name="img" type="file" accept="image/*">
    </div>
    <div>
      <textarea name="caption" placeholder="キャプション(140文字以下)"></textarea>
    </div>
    <div>
      <input type="submit" value="作成">
    </div>
  </form>

  <div>
    <?php foreach($files as $file): ?>
      <img src="<?php echo "{$file['file_path']}" ?>" alt="">
      <p><?php echo h("{$file['description']}") ?></p>
    <?php endforeach ?>
  </div>
</body>
</html>
