#!/usr/bin/env bash
echo unsafe counts:
rg "unsafe[ \{]" ./ --count | rg "(.*)?:(.*)" -o -r "\$2 \$1" | sort --numeric
echo
echo static mut counts:
rg "static mut" ./ --count | rg "(.*)?:(.*)" -o -r "\$2 \$1" | sort --numeric
echo
echo total unsafe:  $(rg "unsafe[ \{]" ./ | wc -l)
echo total static mut: $(rg "static mut" ./ | wc -l)
