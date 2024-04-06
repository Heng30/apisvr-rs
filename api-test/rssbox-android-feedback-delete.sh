#!/bin/bash

curl -X DELETE \
    -H "Authorization: Bearer 654321" \
    localhost:8004/rssbox/android/feedback/$1

