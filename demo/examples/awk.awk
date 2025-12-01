#!/usr/bin/awk -f
BEGIN {
    FS = ","
    print "Processing CSV data..."
}

/^[^#]/ {
    sum += $2
    count++
    names[NR] = $1
}

END {
    avg = count > 0 ? sum / count : 0
    printf "Total: %d, Average: %.2f\n", sum, avg
}
