#!/bin/bash


curl --header "Content-Type: application/json" \
  --request POST \
  --data '{"username":"kotur","password":"qweqwe123", "email":"dzabu"}' \
  http://localhost:3000/user
