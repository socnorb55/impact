data "aws_caller_identity" "current" {}

resource "aws_iam_openid_connect_provider" "github_actions_oidc_provider" {
  url = "https://token.actions.githubusercontent.com"

  client_id_list = [
    "sts.amazonaws.com",
  ]

  thumbprint_list = [
    "6938fd4d98bab03faadb97b34396831e3780aea1",
    "1c58a3a8518e8759bf075b76b750d4f2df264fcd"
  ]
}

resource "aws_iam_policy" "impact_calculator_lambda_policy" {
  name   = "impact-calculator-lambda"
  path   = "/system/runtime/"
  policy = data.aws_iam_policy_document.impact_calculator_lambda_policy.json
}

resource "aws_iam_policy" "impact_iac_policy" {
  name   = "impact-iac"
  path   = "/system/iac/"
  policy = data.aws_iam_policy_document.impac_iac_policy.json
}

data "aws_iam_policy_document" "impact_calculator_lambda_arpd" {
  statement {
    actions = [
      "sts:AssumeRole"
    ]

    principals {
      identifiers = [
        "lambda.amazonaws.com"
      ]
      type = "Service"
    }
  }
}

data "aws_iam_policy_document" "impact_iac_arpd" {
  statement {
    actions = [
      "sts:AssumeRoleWithWebIdentity"
    ]

    condition {
      test     = "StringLike"
      values   = "repo:socnorb55/impact:*"
      variable = "token.actions.githubusercontent.com:sub"
    }

    condition {
      test     = "StringEquals"
      values   = "sts.amazonaws.com"
      variable = "token.actions.githubusercontent.com:aud"
    }

    principals {
      identifiers = [
        "arn:aws:iam::${data.aws_caller_identity.current.account_id}:oidc-provider/token.actions.githubusercontent.com"
      ]
      type = "Federated"
    }
  }
}

data "aws_iam_policy_document" "impact_calculator_lambda_policy" {
  statement {
    actions = [
      "logs:CreateLogGroup",
      "logs:CreateLogStream",
      "logs:PutLogEvents"
    ]

    resources = [
      aws_cloudwatch_log_group.impact_calculator_lambda.arn
    ]
  }

  statement {
    actions = [
      "sqs:SendMessage"
    ]

    resources = [
      aws_sqs_queue.impact_calculator_deadletter_queue.arn
    ]
  }

  statement {
    actions = [
      "xray:PutTraceSegments",
      "xray:PutTelemetryRecords"
    ]

    resources = [
      "*"
    ]
  }
}

data "aws_iam_policy_document" "impact_iac_policy" {

  statement {
    actions = [
      "cloudwatch:*"
    ]

    resources = [
      aws_cloudwatch_alarm.impact_calculator_dead_letter_alarm.arn,
      aws_cloudwatch_log_group.impact_calculator_lambda.arn
    ]
  }

  statement {
    actions = [
      "iam:*Policy*"
    ]

    resources = [
      aws_iam_policy.impact_iac_policy.arn
    ]
  }

  statement {
    actions = [
      "iam:*OpenIDConnect*"
    ]

    resources = [
      aws_iam_openid_connect_provider.github_actions_oidc_provider.arn
    ]
  }

  statement {
    actions = [
      "iam:*Role*"
    ]

    resources = [
      aws_iam_role.impact_iac_role.arn
    ]
  }

  statement {
    actions = [
      "lambda:*"
    ]

    resources = [
      aws_lambda_function.impact_calculator_function.arn
    ]
  }

  statement {
    actions = [
      "s3:CreateBucket",
      "s3:DeleteBucket",
      "s3:GetObject",
      "s3:PutObject",
      "s3:TagResource",
      "s3:UntagReasource"
    ]

    resources = [
      aws_s3_bucket.impact_iac_bucket.arn,
      "${aws_s3_bucket.impact_iac_bucket.arn}/*"
    ]
  }

  statement {
    actions = [
      "sqs:*"
    ]

    resources = [
      aws_sqs_queue.impact_calculator_deadletter_queue.arn
    ]
  }
}

resource "aws_iam_role" "impact_calculator_lambda_role" {
  assume_role_policy    = data.aws_iam_policy_document.impact_calculator_lambda_arpd.arn
  force_detach_policies = true
  name                  = "impact-calculator-lambda"
  path                  = "/system/runtime/"
}

resource "aws_iam_role" "impact_iac_role" {
  assume_role_policy    = data.aws_iam_policy_document.impact_iac_arpd.json
  force_detach_policies = true
  name                  = "impact-iac"
  path                  = "/system/iac/"
}

resource "aws_iam_role_policy_attachment" "impact_calculator_lambda_policy_attachment" {
  role       = aws_iam_role.impact_calculator_lambda_role.name
  policy_arn = aws_iam_policy.impact_calculator_lambda_policy.arn
}

resource "aws_iam_role_policy_attachment" "impact_iac_policy_attachment" {
  role       = aws_iam_role.impact_iac_role.name
  policy_arn = aws_iam_policy.impact_iac_policy.arn
}
