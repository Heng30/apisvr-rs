#!/bin/bash

curl -X DELETE \
    -H "Authorization: Bearer 123456" \
    localhost:8004/rssbox/android/feedback/$1

