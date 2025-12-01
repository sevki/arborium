#!/usr/bin/env fish

set -l greeting "Hello"
set -gx PATH $HOME/bin $PATH

function greet --argument name
    echo "$greeting, $name!"
end

for file in *.txt
    if test -f $file
        echo "Found: $file"
    end
end

greet "World"
