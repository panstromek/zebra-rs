#!/usr/bin/env bash
rg "unsafe[ \{]" ./ --count | rg "(.*)?:(.*)" -o -r "\$2 \$1" | sort --numeric
echo total unsafe:  $(rg "unsafe[ \{]" ./ | wc -l)
echo total static mut: $(rg "static mut" ./ | wc -l)