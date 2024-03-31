#!/bin/bash

curl -X DELETE \
    -H "authorization: Bearer 123456" \
    localhost:8004/rssbox/rss/list/cn/$1
