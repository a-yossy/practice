variable "instance_type" {
  description = "The type of EC2 Instances to run(e.g. t2.micro)"
  type = string

  validation {
    condition = contains(["t2.micro", "t3.micro"], var.instance_type)
    error_message = "Only free tier is allowed: t2.micro | t3.micro"
  }
}

variable "min_size" {
  description = "The minimum number of EC2 Instances in the ASG"
  type = number

  validation {
    condition = var.min_size > 0
    error_message = "ASG's can't be empty or we'll have an outage!"
  }

  validation {
    condition = var.min_size <= 10
    error_message = "ASGs must have 10 or fewer instances to keep costs down."
  }
}

