+++
date = 2023-03-29T07:00:00Z
title = "Getting Started with Terraform: Deploy your first resource"
type = "post"
tags = ["aws", "devops","programming", "iac"]
authors = ["Sylvain Kerkour"]
url = "/iac-getting-started-with-terraform-deploy-resource"
draft = true

[extra]
lang = "en"
+++

Infrastructure as Code (IaC) has revolutionized the way we manage and provision IT infrastructure. One of the most popular IaC tools is Terraform, an open-source tool developed by HashiCorp. Terraform allows you to define, provision, and manage infrastructure across multiple cloud providers using a simple and human-readable language called HashiCorp Configuration Language (HCL). In this blog post, we'll walk you through the process of getting started with Terraform, complete with multiple code examples, including deploying a Docker container on AWS EC2 using Terraform.


## Why Infrastructure as Code

**Consistency and repeatability**: IaC enables you to define your infrastructure in code, which can be version-controlled and shared across teams. This ensures that your infrastructure is consistently and reliably provisioned, avoiding discrepancies that may arise from manual processes or ad-hoc configurations. With IaC, you can easily replicate your infrastructure across multiple environments, such as development, staging, and production, ensuring that these environments remain consistent with each other.

**Speed and efficiency**: IaC allows you to automate the provisioning and management of infrastructure, which greatly reduces the time and effort required to set up and maintain your IT resources. By automating repetitive and error-prone tasks, IaC helps increase the speed of deployment and reduces the risk of human error. This results in more efficient operations and faster time to market for your applications and services.

**Improved collaboration and maintainability**: IaC encourages collaboration between different teams, such as development, operations, and security, by providing a single source of truth for your infrastructure. With IaC, changes to the infrastructure are made in code and can be reviewed, tested, and approved using standard software development practices like code reviews and automated testing. This promotes better communication and shared understanding among team members, leading to more maintainable and resilient infrastructure.


## Installing Terraform

Before you can use Terraform, you'll need to install it on your machine. Terraform is available for various platforms, including Windows, macOS, and Linux. To install Terraform, follow the steps below:

- Download the appropriate package for your operating system from the [Terraform downloads page](https://www.terraform.io/downloads.html).
- Extract the downloaded archive.
- Move the extracted terraform binary to a directory in your system's `PATH`, such as `/usr/local/bin` on macOS and Linux or `C:\Windows\system32` on Windows.
- Verify the installation by opening a terminal and running the following command:

```shell
$ terraform --version
```

You should see the Terraform version displayed in the output.

## Configuring your AWS credentials

To manipulate AWS resources using Terraform you first need to configure your credentials.

- Create an access key in the [IAM console](https://us-east-1.console.aws.amazon.com/iamv2/home).
- [Configure the AWS CLI](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-quickstart.html) and enter the newly created Access Key:

```shell
$ aws configure
```

Your are now ready to roll 🚀


## Basic Terraform Concepts


Before diving into the code examples, let's familiarize ourselves with some basic Terraform concepts:

**Providers**: Terraform uses providers to interact with different infrastructure platforms, such as AWS, Azure, and GCP. You'll need to configure a provider in your Terraform configuration file to start managing resources on your chosen platform.

**Resources**: Resources are the building blocks of your infrastructure, such as virtual machines, security groups, and load balancers. In Terraform, you define resources using HCL.

**Variables**: Variables allow you to parameterize your Terraform configurations, making them more flexible and reusable. You can pass variables as arguments to your Terraform configuration or store them in separate files.

**Outputs**: Outputs define the values that should be returned after your Terraform configuration has been applied. This is useful for extracting information about your infrastructure for further use, such as the IP address of a created virtual machine.

**State**: Terraform uses a state file to keep track of the infrastructure it has created. This file maps your Terraform configuration to the real-world resources, allowing Terraform to detect and manage changes.


## Your First Terraform Configuration

Now that we've covered the basic concepts, let's create a simple Terraform configuration to provision an AWS S3 bucket. First, create a new directory for your Terraform configuration and navigate to it in your terminal.

Next, create a new file named main.tf and add the following code:

```hcl
provider "aws" {
  region = "us-west-2"
}

resource "aws_s3_bucket" "my_bucket" {
  bucket = "my-terraform-bucket-12345"
}
```

This configuration specifies the AWS provider and sets the region to `us-west-2`. It also defines an `aws_s3_bucket` resource named `my_bucket` with the specified bucket name.


## Initializing the Terraform Configuration

Before you can apply your configuration, you'll need to initialize your Terraform working directory by running the following command:

```shell
$ terraform init
```

This command downloads the necessary provider plugins and sets up the backend for storing your Terraform state.

## Planning and Applying the Terraform Configuration


To preview the changes that will be made to your infrastructure, run the `terraform plan` command:

```shell
$ terraform plan
```

This command will show you a detailed summary of the resources that will be created, modified, or destroyed. Review the output to ensure it aligns with your intended infrastructure changes.

Once you're satisfied with the plan, you can apply the changes by running the following command:

```shell
$ terraform apply
```

Terraform will prompt you to confirm that you want to apply the changes. Type "yes" and press Enter to proceed. Terraform will then create the specified S3 bucket and output a summary of the changes made.

## Destroying the Infrastructure


When you no longer need the infrastructure created by your Terraform configuration, you can destroy it using the following command:

```bash
$ terraform destroy
```

This command will prompt you to confirm that you want to destroy the infrastructure. Type "yes" and press Enter to proceed. Terraform will delete the S3 bucket and output a summary of the changes made.



## Some closing thoughts

And that's all, really. IaC is a powerful tool and will save you hours of painful debugging by creating reproducible environements.
