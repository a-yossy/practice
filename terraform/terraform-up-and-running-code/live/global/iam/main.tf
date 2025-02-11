provider "aws" {
  region = "us-east-2"
}

module "users" {
  source    = "../../../modules/landing-zone/iam-user"
  for_each  = toset(var.user_names)
  user_name = each.value
  # give_neo_cloudwatch_full_access = false
}
