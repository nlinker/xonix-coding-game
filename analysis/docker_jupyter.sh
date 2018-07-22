#!/usr/bin/env bash

docker run -it -p 5022:22 -p 4545:4545 -v "${PWD}":/notebooks -w /notebooks festline/mlcourse_open jupyter
