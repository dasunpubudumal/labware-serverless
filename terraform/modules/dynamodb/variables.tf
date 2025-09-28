variable "table_name" {
    description = "Name of the DynamoDB table"
    type = string
}

variable "partition_key" {
  description = "Partition key (aka. hash key) of the table"
  type = string
}

variable "attributes" {
  description = "Attributes for the given table"
  type = list(object({
    name = string
    type = string
  }))
}