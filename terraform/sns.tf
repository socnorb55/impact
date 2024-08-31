resource "aws_sns_topic" "incident_notifier_topic" {
  name = "incident-notifier-topic"
}

resource "aws_sns_topic_subscription" "incident_notifier_subscription" {
  topic_arn = aws_sns_topic.incident_notifier_topic.arn
  protocol  = "email"
  endpoint  = "socnorb55+support@gmail.com"
}
