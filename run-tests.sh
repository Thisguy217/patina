#!/bin/bash

buckets="20 40 60 80 100"

numbers_20="21 22 23"
numbers_40="41 42 43 44 45 46"
numbers_60="61 62 64 66 67 68"
numbers_80="81 82 83 84 85 86"
numbers_100="101 102 103 104 105 108"

testdir="testing"
diffopts="-a -i -b -w -B"

# Build once
cargo build --quiet
# Determine binary name from Cargo.toml package name
PROGRAM=$(basename "$(pwd)")
BINARY="./target/debug/$PROGRAM"

for bucket in $buckets; do

    echo "Bucket $bucket"
    eval numbers=\$numbers_$bucket

    for number in $numbers; do

        echo "Running input $number"

        inputfile="$testdir/$bucket/input$number.dl"
        answerfile="$testdir/$bucket/answer$number.txt"
        outputfile="actual$number.txt"

        "$BINARY" "$inputfile" > "$outputfile"

        diff $diffopts "$answerfile" "$outputfile" || \
            echo "diff failed on test $number"

        rm "$outputfile"

    done
done