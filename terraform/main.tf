# DynamoDB documentation: 
# https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.CoreComponents.html

# === Checks which account is being used ===
data "aws_caller_identity" "current" {}
# === END ===

#  === Defines the resources ===
module "dynamodb_tables" {
  source         = "./modules/dynamodb"
  billing_mode   = "PROVISIONED"
  read_capacity  = 1
  write_capacity = 1
}

# === END ===