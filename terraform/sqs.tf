resource "aws_sqs_queue" "impact_calculator_deadletter_queue" {
  name = "impact-calculator-lambda-deadletter-queue"
}