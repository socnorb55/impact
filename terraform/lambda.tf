resource "aws_lambda_function" "impact_calculator_function" {
  filename      = "calculator_lambda.zip"
  function_name = "impact-calculator"
  role          = aws_iam_role.impact_calculator_lambda_role.arn
  handler       = "src.main"

  source_code_hash = data.archive_file.lambda.output_base64sha256

  runtime = "nodejs18.x"

  environment {
    variables = {
      foo = "bar"
    }
  }
}