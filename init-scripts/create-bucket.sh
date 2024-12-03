#!/bin/bash
set -e # Exit immediately if a command exits with a non-zero status
awslocal s3 mb s3://archive
