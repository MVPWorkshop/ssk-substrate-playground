#!/bin/bash
set -e  # Exit immediately if a command exits with a non-zero status

echo "Creating S3 bucket 'my-bucket'..."
awslocal s3 mb s3://my-bucket
