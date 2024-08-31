resource "aws_s3_bucket" "impact_iac_bucket" {
  bucket = "impact-${var.env}-iac-useast1"
}