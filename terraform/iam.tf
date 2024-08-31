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

data "aws_iam_policy_document" "impac_iac_arpd" {
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

resource "aws_iam_policy" "impact_iac_policy" {
  name   = "impact-iac"
  path   = "/system/iac/"
  policy = data.aws_iam_policy_document.impac_iac_policy.json
}

data "aws_iam_policy_document" "impac_iac_policy" {
  statement {
    actions = [
      "iam:AddClientIDToOpenIDConnectProvider",
      "iam:CreateOpenIDConnectProvider",
      "iam:DeleteOpenIDConnectProvider",
      "iam:GetOpenIDConnectProvider",
      "iam:ListOpenIDConnectProviderTags",
      "iam:ListOpenIDConnectProviders",
      "iam:RemoveClientIDFromOpenIDConnectProvider",
      "iam:TagOpenIDConnectProvider",
      "iam:UntagOpenIDConnectProvider",
      "iam:UpdateOpenIDConnectProviderThumbprint"
    ]

    resources = [
      aws_iam_openid_connect_provider.github_actions_oidc_provider.arn
    ]
  }

  statement {
    actions = [
      "iam:AttachRolePolicy",
      "iam:CreateRole",
      "iam:DeleteRole",
      "iam:DetachRolePolicy",
      "iam:GetRole",
      "iam:ListAttachedRolePolicies",
      "iam:ListRoles",
      "iam:ListRoleTags",
      "iam:TagRole",
      "iam:UntagRole",
      "iam:UpdateRole"
    ]

    resources = [
      aws_iam_role.impact_iac_role.arn
    ]
  }

  statement {
    actions = [
      "iam:CreatePolicy",
      "iam:CreatePolicyVersion",
      "iam:DeletePolicy",
      "iam:GetPolicy",
      "iam:GetPolicyVersion",
      "iam:ListPolicies",
      "iam:ListPolicyTags",
      "iam:SetDefaultPolicyVersion",
      "iam:TagPolicy",
      "iam:UntagPolicy",
      "iam:UpdatePolicy"
    ]

    resources = [
      aws_iam_policy.impact_iac_policy.arn
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
}

resource "aws_iam_role" "impact_iac_role" {
  assume_role_policy    = data.aws_iam_policy_document.impac_iac_arpd.json
  force_detach_policies = true
  name                  = "impact-iac"
  path                  = "/system/iac/"
}

resource "aws_iam_role_policy_attachment" "impact_iac_policy_attachment" {
  role       = aws_iam_role.impact_iac_role.name
  policy_arn = aws_iam_policy.impact_iac_policy.arn
}
