BEGIN {
    in_preset = 0
    in_claude_plugins = 0
    in_deactivate = 0
}
{
    match($0, /^ */)
    line_indent = RLENGTH

    if (in_preset == 0 && $0 ~ /^  [a-z-]+:$/ && $0 ~ preset ":") {
        in_preset = 1
        next
    }

    if (in_preset == 1 && line_indent == 2 && $0 ~ /^  [a-z-]+:$/) {
        exit
    }

    if (in_preset == 1 && $0 ~ /claude_plugins:$/) {
        in_claude_plugins = 1
        next
    }

    if (in_claude_plugins == 1 && line_indent == 4 && $0 ~ /^[a-z]+:$/) {
        in_claude_plugins = 0
        in_deactivate = 0
    }

    if (in_claude_plugins == 1 && $0 ~ /deactivate:$/) {
        in_deactivate = 1
        next
    }

    if (in_deactivate == 1 && $0 ~ /^[[:space:]]+-/) {
        plugin = substr($0, index($0, "-") + 1)
        gsub(/^[[:space:]]+|[[:space:]]+$/, "", plugin)
        if (length(plugin) > 0) {
            if (length(result) > 0) {
                result = result ", \"" plugin "\""
            } else {
                result = "\"" plugin "\""
            }
        }
    }
}
END {
    if (length(result) > 0) {
        print "[" result "]"
    } else {
        print "[]"
    }
}
