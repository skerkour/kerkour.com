+++
date = 2023-03-20T07:00:00Z
title = "Serverless Rust: Write, build and deploy your first lambda function"
type = "post"
tags = ["rust", "aws", "serverless","programming"]
authors = ["Sylvain Kerkour"]
url = "/serverless-rust-deploy-aws-lambda-function"
draft = true

[extra]
lang = "en"
+++


Serverless computing has gained significant popularity in recent years, allowing developers to run their code without having to manage servers. AWS Lambda is one of the most popular serverless platforms, providing automatic scaling and reducing operational costs and complexity.

While I'm pretty skeptic about [Serverless and its pay-as-you-go model](https://kerkour.com/denial-of-wallet-attacks-the-new-ddos) which may bankrupt you if you make a small mistake or someone want to "Denial-of-Wallet" you, I found some really interesting usage of it in the wild, so here is a guide to to write a Lambda function in Rust and deploy it on AWS Lambda using the terminal.

## Prerequisites

Before diving into the code, ensure that you have the following prerequisites set up:

- An AWS account: [Sign up for a free AWS account](https://aws.amazon.com) if you don't have one already. You'll need it to access AWS Lambda and other AWS services.
- AWS CLI: [Install the AWS Command Line Interface (CLI)](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html) and [configure it with your AWS account credentials](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-quickstart.html).
- Rust and Cargo: Install Rust and Cargo, Rust's package manager, on your system with [rustup](http://rustup.rs).
- Docker: Install Docker to build the Rust Lambda function using a container.

## Creating your first Rust Lambda Function

To create a Rust Lambda function, follow these steps:


## Building the Rust Lambda Function


## Packaging and Deploying the Rust Lambda Function


## Updating and Redeploying the Rust Lambda Function

## Destroying your lambda function

Remember to delete the Lambda function after you're done experimenting to avoid potential costs:

```shell
$ aws lambda delete-function --function-name MyFirstRustLambdaFunction
```


## Some closing thoughts

In this blog post, we demonstrated how to create a Rust Lambda function, build it using Docker, and deploy it to AWS Lambda.

Rust is an excellent choice for serverless applications, thanks to its reliability and performance features.

By combining Rust with AWS Lambda, you can develop highly efficient and scalable serverless applications suitable for your task requiring a lot of compute for a small amount of time.


**Want to learn more about Rust, applied Cryptography and Security? Take a look at my book [Black Hat Rust](https://kerkour.com/black-hat-rust) where we build web services and web apps using Rust and WebAssembly.**
