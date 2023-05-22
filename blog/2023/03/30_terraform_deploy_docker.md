+++
date = 2023-03-30T07:00:00Z
title = "Deploy a Docker container on AWS EC2 with terraform"
type = "post"
tags = ["aws", "devops","programming", "iac"]
authors = ["Sylvain Kerkour"]
url = "/iac-deploy-a-docker-container-on-aws-ec2-with-terraform"
draft = true

[extra]
lang = "en"
+++

[Two days ago](/iac-getting-started-with-terraform-deploy-resource) we saw how to deploy your first resource with Terraform.

Now let's dive into a more complex example, deploying a Docker container on an AWS EC2 instance using Terraform. In this example, we'll use the following AWS resources:

- A security group
- An IAM instance profile
- An EC2 instance

Create a new directory for this example, and create a new main.tf file within it. First, configure the AWS provider and create a security group to allow incoming traffic on port 80:

```hcl
provider "aws" {
  region = "us-east-1"
}

resource "aws_security_group" "docker_sg" {
  name        = "docker_sg"
  description = "Allow inbound traffic on port 80"

  ingress {
    from_port   = 80
    to_port     = 80
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }
}
```

Next, create an IAM role and instance profile for the EC2 instance:

```hcl
resource "aws_iam_role" "ec2_role" {
  name = "ec2_role"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Action = "sts:AssumeRole"
        Effect = "Allow"
        Principal = {
          Service = "ec2.amazonaws.com"
        }
      }
    ]
  })
}

resource "aws_iam_instance_profile" "ec2_instance_profile" {
  name = "ec2_instance_profile"
  role = aws_iam_role.ec2_role.name
}
```

Now, create an EC2 instance running Amazon Linux 2 and install Docker on it using a user data script:

```hcl
resource "aws_instance" "docker_instance" {
  ami           = "ami-0c55b159cbfafe1f0" # Amazon Linux 2 AMI
  instance_type = "t2.micro"

  key_name               = "your-key-pair-name"
  vpc_security_group_ids = [aws_security_group.docker_sg.id]
  iam_instance_profile   = aws_iam_instance_profile.ec2_instance_profile.name

  user_data = <<-EOF
              #!/bin/bash
              yum update -y
              amazon-linux-extras install docker -y
              systemctl start docker
              systemctl enable docker
              docker run -d -p 80:80 nginx
              EOF

  tags = {
    Name = "Docker Instance"
  }
}
```

Replace `your-key-pair-name` with the name of an existing EC2 key pair in your AWS account. The user data script installs Docker, starts the Docker service, and runs an Nginx container that listens on port 80.

Finally, add an output block to display the public IP address of the created EC2 instance:

```hcl
output "instance_public_ip" {
  value = aws_instance.docker_instance.public_ip
}
```


Your complete `main.tf` file should now look like this:

```hcl
provider "aws" {
  region = "us-east-1"
}

resource "aws_security_group" "docker_sg" {
  name        = "docker_sg"
  description = "Allow inbound traffic on port 80"

  ingress {
    from_port   = 80
    to_port     = 80
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }
}

resource "aws_iam_role" "ec2_role" {
  name = "ec2_role"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Action = "sts:AssumeRole"
        Effect = "Allow"
        Principal = {
          Service = "ec2.amazonaws.com"
        }
      }
    ]
  })
}

resource "aws_iam_instance_profile" "ec2_instance_profile" {
  name = "ec2_instance_profile"
  role = aws_iam_role.ec2_role.name
}

resource "aws_instance" "docker_instance" {
  ami           = "ami-005f9685cb30f234b" # Amazon Linux 2 AMI, use ami-05a66dc4a507a82cc for arm64
  instance_type = "t2.micro" # use t4g.micro for arm64

  key_name               = "your-key-pair-name"
  vpc_security_group_ids = [aws_security_group.docker_sg.id]
  iam_instance_profile   = aws_iam_instance_profile.ec2_instance_profile.name

  user_data = <<-EOF
              #!/bin/bash
              yum update -y
              amazon-linux-extras install docker -y
              systemctl start docker
              systemctl enable docker
              docker run -d -p 80:80 nginx
              EOF

  tags = {
    Name = "Docker Instance"
  }
}

output "instance_public_ip" {
  value = aws_instance.docker_instance.public_ip
}
```


Initialize your Terraform working directory, plan your changes, and apply your configuration:

```shell
$ terraform init
$ terraform plan
$ terraform apply
```

After applying the configuration, Terraform will output the public IP address of the EC2 instance. You can access the Nginx container by navigating to this IP address in your web browser.

When you no longer need the infrastructure, destroy it by running `terraform destroy`.

And that's all!
