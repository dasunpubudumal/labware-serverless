# Defines DynamoDB resources

# Tables
resource "aws_dynamodb_table" "location_types" {
  name           = var.table_name
  billing_mode   = "PROVISIONED"
  read_capacity  = 1
  write_capacity = 1
  hash_key       = var.partition_key

  dynamic "attribute" {
    for_each = var.attributes  
    content {
      name = attribute.value.name
      type = attribute.value.type
    }
  }
}