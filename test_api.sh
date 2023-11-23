#!/bin/bash


curl --header "Content-Type: application/json" \
  --verbose \
  --request POST \
  --data '{"username":"kotur","password":"qweqwe123", "email":"dzabu@dza.bu"}' \
  http://localhost:3000/print_user
