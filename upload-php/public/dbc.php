<?php
  function dbc() {
    $host = 'localhost';
    $dbname = 'file_db';
    $user = 'root';
    $pass = 'root';

    $dns = "mysql:host=$host;dbname=$dbname;charset=utf8";

    try {
      $pdo = new PDO($dns, $user, $pass,
        [
          PDO::ATTR_ERRMODE => PDO::ERRMODE_EXCEPTION,
          PDO::ATTR_DEFAULT_FETCH_MODE => PDO::FETCH_ASSOC
        ]
      );
      return $pdo;
    } catch (PDOException $e) {
      exit($e->getMessage());
    }
  }

  function fileSave($fileName, $savePath, $caption) {
    $result = false;
    $sql = "INSERT INTO
              file_table (file_name, file_path, description)
            VALUE
              (?, ?, ?)";
    try {
      $stmt = dbc()->prepare($sql);
      $stmt->bindValue(1, $fileName);
      $stmt->bindValue(2, $savePath);
      $stmt->bindValue(3, $caption);
      $result = $stmt->execute();
      return $result;
    } catch (Exception $e) {
      echo $e->getMessage();
      return $result;
    }
  }

  function getAllFile() {
    $sql = "SELECT * FROM file_table";
    $fileData = dbc()->query($sql);

    return $fileData;
  }

  function h($s) {
    return htmlspecialchars($s, ENT_QUOTES, "UTF-8");
  }
?>
