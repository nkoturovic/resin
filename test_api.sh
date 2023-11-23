#!/bin/bash


curl --header "Content-Type: application/json" \
  --verbose \
  --request POST \
  --data '{"username":"kotur","password":"qweqwe123", "email":"dzabu2@dza.bu"}' \
  http://localhost:3000/user
