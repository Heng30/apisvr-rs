#!/bin/bash

random_name=`tr -dc A-Za-z0-9 < /dev/urandom | head -c 10`
random_data=`tr -dc A-Za-z0-9 < /dev/urandom | head -c 10`
data='{"name":"'$random_name'", "url":"'${random_data}'"}'

curl -X POST \
    -H "Content-Type: application/json" \
    -d "$data" \
    localhost:8004/rssbox/rss/list/cn
