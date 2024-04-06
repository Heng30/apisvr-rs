#!/bin/bash

detail_str_1=`tr -dc A-Za-z0-9 < /dev/urandom | head -c 100`
detail_str_2=`tr -dc A-Za-z0-9 < /dev/urandom | head -c 80`
detail_str_3=`tr -dc A-Za-z0-9 < /dev/urandom | head -c 70`
detail_str_4=`tr -dc A-Za-z0-9 < /dev/urandom | head -c 90`
detail_str_5=`tr -dc A-Za-z0-9 < /dev/urandom | head -c 50`
detail_str_6=`tr -dc A-Za-z0-9 < /dev/urandom | head -c 80`

detail=${detail_str_1}"\n"$detail_str_2"\n"${detail_str_3}"\n"$detail_str_4"\n"${detail_str_5}"\n"$detail_str_6"\n"

data='{"latest_version": "v1.1.0", "detail_cn":"'${detail}'",  "detail_en": "'${detail}'", "url": "https://heng30.xyz"}'

echo $data

curl -X POST \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer 654321" \
    -d "$data" \
    localhost:8004/latest/version?q=test


