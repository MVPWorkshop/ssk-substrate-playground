#!/bin/bash
set -e # Exit immediately if a command exits with a non-zero status

echo "Creating S3 bucket 'templates'..."
awslocal s3 mb s3://templates
echo "Copying recursivly ./templates to s3://templates ..."
awslocal s3 cp /etc/app/templates s3://templates --recursive
