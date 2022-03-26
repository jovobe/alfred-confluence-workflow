#!/bin/bash

ARCH=$(uname -p)
if [ $ARCH == "arm" ]; then
  ./acw-arm "$@"
else
  ./acw-x86 "$@"
fi
