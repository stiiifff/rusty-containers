#!/usr/bin/env bash

docker build --platform linux/amd64 -t simple-api:latest -f ./simple-api/Dockerfile ./simple-api
docker build --platform linux/amd64 -t simple-worker:latest -f ./simple-worker/Dockerfile ./simple-worker
