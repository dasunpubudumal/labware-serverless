# DynamoDB documentation: 
# https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.CoreComponents.html

# === Checks which account is being used ===
data "aws_caller_identity" "current" {}
# === END ===

#  === Defines the resources ===
module "dynamodb_tables" {
  source = "./modules/dynamodb"

  table_name    = "location_types"
  partition_key = "name"

  attributes = [{
    "name" : "name",
    "type" : "S"
  }]
}
# === END ===