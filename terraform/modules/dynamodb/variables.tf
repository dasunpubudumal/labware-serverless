variable "billing_mode" {
  description = "Billing mode"
  type = string
  default = "PROVISIONED"
}

variable "read_capacity" {
  default = 1
  description = "DynamoDB Read Capacity"
  type = number
}

variable "write_capacity" {
  default = 1
  description = "DynamoDB Write Capacity"
  type = number
}