#!/bin/bash

random_str=`tr -dc A-Za-z0-9 < /dev/urandom | head -c 10`
data='{"appid": "test", "type": "feedback", "data":'${random_str}'}'

curl -X POST \
    -H "Content-Type: application/json" \
    -d "$data" \
    localhost:8004/rssbox/android/feedback


