terraform {

  backend "s3" {
    key    = "impact/tf.tfstate"
    region = "us-east-1"
  }

  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.65"
    }
  }
}

provider "aws" {
  region  = "us-east-1"
}
