#!/bin/bash


curl -v --header "Content-Type: application/json" \
  --request POST \
  --data '{"username":"kotur","password":"qweqwe123"}' \
  http://localhost:3000/user
