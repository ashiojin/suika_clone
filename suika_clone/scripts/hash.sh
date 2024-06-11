#!/bin/bash

cd $1
find . -type f | sort | xargs sha256sum | sha256sum | awk '{ print $1 }'
