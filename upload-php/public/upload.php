<?php
  require_once('./dbc.php');

  $file = $_FILES['img'];
  $fileName = basename($file['name']);
  $tmpPath = $file['tmp_name'];
  $fileErr = $file['error'];
  $fileSize = $file['size'];
  $uploadDir = '/Applications/MAMP/htdocs/upload/images/';
  $saveFileName = date('YmdHis').$fileName;
  $errMsgs = array();
  $savePath = $uploadDir.$saveFileName;
  $caption = filter_input(INPUT_POST, 'caption', FILTER_SANITIZE_SPECIAL_CHARS);

  if (empty($caption)) {
    array_push($errMsgs, 'キャプションを入力して下さい');
  }

  if (strlen($caption) > 140) {
    array_push($errMsgs, 'キャプションは140文字以内で入力して下さい');
  }

  if ($fileSize > 1048576 || $fileErr == 2) {
    array_push($errMsgs, 'ファイルサイズは1MB未満にして下さい');
  }

  $allowExt = array('jpg', 'jpeg', 'png');
  $fileExt = pathinfo($fileName, PATHINFO_EXTENSION);
  if (!in_array(strtolower($fileExt), $allowExt)) {
    array_push($errMsgs, '画像ファイルを添付して下さい');
  }

  if (count($errMsgs) === 0) {
    if (is_uploaded_file($tmpPath)) {
      if (move_uploaded_file($tmpPath, $savePath)) {
        echo $fileName.'を'.$uploadDir.'にアップしました'.'<br>';
        $result = fileSave($fileName, $savePath, $caption);
        if ($result) {
          echo 'データベースに保存しました';
        } else {
          echo 'データベースへの保存が失敗しました';
        }
      } else {
        echo 'ファイルを保存できませんでした';
      }
    } else {
      echo 'ファイルが選択されていません'.'<br>';
    }
  } else {
    foreach($errMsgs as $msg) {
      echo $msg.'<br>';
    }
  }
?>

<a href='/upload/public/form.php'>戻る</a>
