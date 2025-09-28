# Defines DynamoDB resources
# Note: We are defining only the key schema within `attributes` section.

# Tables
resource "aws_dynamodb_table" "location_types" {
  name           = "location_types"
  billing_mode   = var.billing_mode
  read_capacity  = var.read_capacity
  write_capacity = var.write_capacity
  hash_key       = "name"

  attribute {
    name = "name"
    type = "S"
  }
}

resource "aws_dynamodb_table" "locations" {
  name           = "locations"
  billing_mode   = var.billing_mode
  read_capacity  = var.read_capacity
  write_capacity = var.write_capacity
  hash_key       = "name"
  range_key      = "location_type"

  attribute {
    name = "name"
    type = "S"
  }
  attribute {
    name = "location_type"
    type = "S"
  }
}

resource "aws_dynamodb_table" "labware" {
  name           = "labware"
  billing_mode   = var.billing_mode
  read_capacity  = var.read_capacity
  write_capacity = var.write_capacity
  hash_key       = "barcode"
  range_key      = "location_name"

  attribute {
    name = "barcode"
    type = "S"
  }
  attribute {
    name = "location_name"
    type = "S"
  }
}

