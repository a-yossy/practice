<?php
  require_once('dbc.php');

  class Todo extends Dbc {
    protected $tableName = 'todo';

    public function todoCreate($todoParams) {
      $sql = "INSERT INTO
                $this->tableName(title, priority, type, body)
              VALUES
                (:title, :priority, :type, :body)";
      $dbh = $this->dbConnect();
      $dbh->beginTransaction();
      try {
        $stmt = $dbh->prepare($sql);
        $stmt->bindValue(':title', $todoParams['title'], PDO::PARAM_STR);
        $stmt->bindValue(':priority', $todoParams['priority'], PDO::PARAM_STR);
        $stmt->bindValue(':type', $todoParams['type'], PDO::PARAM_STR);
        $stmt->bindValue(':body', $todoParams['body'], PDO::PARAM_STR);
        $stmt->execute();
        $dbh->commit();
        echo 'ToDoを作成しました';
      } catch (PDOException $e) {
        $dbh->rollBack();
        exit($e->getMessage());
      }
    }

    public function todoUpdate($todoParams) {
      $sql = "UPDATE $this->tableName SET
                title = :title, priority = :priority, type = :type, body = :body
              WHERE
                id = :id";
      $dbh = $this->dbConnect();
      $dbh->beginTransaction();
      try {
        $stmt = $dbh->prepare($sql);
        $stmt->bindValue(':title', $todoParams['title'], PDO::PARAM_STR);
        $stmt->bindValue(':priority', $todoParams['priority'], PDO::PARAM_STR);
        $stmt->bindValue(':type', $todoParams['type'], PDO::PARAM_STR);
        $stmt->bindValue(':body', $todoParams['body'], PDO::PARAM_STR);
        $stmt->bindValue(':id', (int)$todoParams['id'], PDO::PARAM_INT);
        $stmt->execute();
        $dbh->commit();
        echo 'ToDoを更新しました';
      } catch (PDOException $e) {
        $dbh->rollBack();
        exit($e->getMessage());
      }
    }

    public function todoValidate($todoParams) {
      if (empty($todoParams['title'])) {
        exit('タイトルを入力して下さい');
      }

      if (mb_strlen($todoParams['title']) > 20) {
        exit('タイトルは20文字以下にして下さい');
      }

      if (empty($todoParams['priority'])) {
        exit('優先度を選択して下さい');
      }

      if (empty($todoParams['type'])) {
        exit('予定種別を選択して下さい');
      }

      if (empty($todoParams['body'])) {
        exit('予定詳細を入力して下さい');
      }
    }
  }
?>
