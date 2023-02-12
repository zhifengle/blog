#!/usr/bin/env bash
set -x
set -eo pipefail

DB_ROOT_PASSWORD=${MYSQL_ROOT_PASSWORD:=secret}
DB_USER=${MYSQL_USER:=testuser}
DB_PASSWORD="${MYSQL_PASSWORD:=password}"
DB_NAME="${MYSQL_DB:=bangumi}"
DB_PORT="${MYSQL_PORT:=3306}"

docker run --name mysql8 \
    -e MYSQL_ROOT_PASSWORD=${DB_ROOT_PASSWORD} \
    -e MYSQL_DATABASE=${DB_NAME} \
    -e MYSQL_USER=${DB_USER} \
    -e MYSQL_PASSWORD=${DB_PASSWORD} \
    -p "${DB_PORT}":3306 \
    -d mysql