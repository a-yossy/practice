<?php
  require_once('../config/env.php');

  class Dbc{
    protected $tableName;

    protected function dbConnect() {
      $host = DB_HOST;
      $dbname = DB_NAME;
      $user = DB_USER;
      $pass = DB_PASS;
      $dsh = "mysql:host=$host;dbname=$dbname;charset=utf8";

      try {
        $dbh = new PDO($dsh, $user, $pass, [
          PDO::ATTR_ERRMODE => PDO::ERRMODE_EXCEPTION,
          PDO::ATTR_DEFAULT_FETCH_MODE => PDO::FETCH_ASSOC,
        ]);
      } catch (PDOException $e) {
        header('Content-Type: text/plain; charset=UTF-8', true, 500);
        exit($e->getMessage());
      }

      return $dbh;
    }

    public function getAll() {
      $dbh = $this->dbConnect();
      $sql = "SELECT * FROM  $this->tableName";
      $stmt = $dbh->query($sql);
      $result = $stmt->fetchall();
      return $result;
    }

    public function getById($id) {
      if (empty($id)) {
        exit('IDが不正です');
      }

      $dbh = $this->dbConnect();

      $stmt = $dbh->prepare("SELECT * FROM $this->tableName where id = :id");
      $stmt->bindValue(':id', (int)$id, PDO::PARAM_INT);
      $stmt->execute();
      $result = $stmt->fetch();

      if (!$result) {
        exit('ToDoがありません');
      }

      return $result;
    }

    public function delete($id) {
      if (empty($id)) {
        exit('IDが不正です');
      }

      $dbh = $this->dbConnect();
      $stmt = $dbh->prepare("DELETE FROM $this->tableName Where id = :id");
      $stmt->bindValue(':id', (int)$id, PDO::PARAM_INT);
      $stmt->execute();
      echo 'ToDoを削除しました';
    }
  }
?>
