resource "aws_lambda_function" "impact_calculator_function" {
  architectures    = ["arm64"]
  filename         = "${path.module}/../build.zip"
  function_name    = "impact-calculator"
  handler          = "src.main"
  memory_size      = 1024
  role             = aws_iam_role.impact_calculator_lambda_role.arn
  runtime          = "provided.al2023"
  source_code_hash = "${path.module}/../build.zip"
  timeout          = 900

  dead_letter_config {
    target_arn = aws_sqs_queue.impact_calculator_deadletter_queue.arn
  }

  environment {
    variables = {
      foo = "bar"
    }
  }

  logging_config {
    log_format = "Text"
    log_group  = aws_cloudwatch_log_group.impact_calculator_lambda.arn
  }

  tracing_config {
    mode = "Active"
  }
}
