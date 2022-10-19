#!/usr/bin/env bash

if [[ ! `command -v grass` ]]; then cargo install grass; fi

if [ ! -f style.css ] || [ style.scss -nt style.css ]; then
    echo "compiling stylesheet"
    grass style.scss > style.css
fi

