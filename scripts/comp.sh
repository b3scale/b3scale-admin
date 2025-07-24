#!/usr/bin/env bash

if [[ ! `command -v grass` ]]; then cargo install grass; fi

if [ ! -f style.css ] || [ style.scss -nt style.css ]; then
    echo "compiling stylesheet"
fi

grass style.scss > style.css
