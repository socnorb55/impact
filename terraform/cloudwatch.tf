resource "aws_cloudwatch_log_group" "impact_calculator_lambda" {
  name              = "impact-calculator-lambda"
  retention_in_days = 90
}

resource "aws_cloudwatch_metric_alarm" "impact_calculator_lambda_alarm" {
  alarm_actions = [
    aws_sns_topic.incident_notifier_topic.arn
  ]
  alarm_name          = "impact-calculator-lambda-dead-letter-alarm"
  comparison_operator = "GreaterThanOrEqualToThreshold"
  dimensions = {
    QueueName = aws_sqs_queue.impact_calculator_deadletter_queue.arn
  }
  evaluation_periods = 2
  metric_name        = "ApproximateNumberOfMessagesVisible"
  namespace          = "AWS/SQS"
  period             = 600
  statistic          = "Sum"
  threshold          = 1
}
