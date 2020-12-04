#!/bin/bash

ERL_DIR=$(asdf where erlang)
[ ! -f erlang.TAGS ] && ctags --recurse -e -f erlang.TAGS $ERL_DIR/lib
ctags --recurse -e -f app.TAGS src test
cat erlang.TAGS app.TAGS > TAGS
rm app.TAGS
