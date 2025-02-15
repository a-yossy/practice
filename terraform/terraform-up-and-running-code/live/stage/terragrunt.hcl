remote_state {
  backend = "s3"

  generate = {
    path = "backend.tf"
    if_exists = "overwrite"
  }

  config = {
    bucket = "bucket"
    key = "${path_relative_to_include()}/terraform.tfstate"
    region = "us-east-2"
    encrypt = true
    dynamodb_table = "table"
  }
}
