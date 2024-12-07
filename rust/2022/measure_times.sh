#!/bin/sh

# Create a file to store the times
echo "| Day | Part 1 | Part 2 |" > times.md
echo "| --- | --- | --- |" >> times.md

for i in $(seq -w 1 25)
do
    sed -i '' "s/use day_.. as today;/use day_${i} as today;/g" src/solutions.rs
    cargo build

    # Capture the output of the time command for part 1
    time1=$( (time cargo run . -p1) 2>&1 | grep real | awk '{print $2}')
    # Capture the output of the time command for part 2
    time2=$( (time cargo run . -p2) 2>&1 | grep real | awk '{print $2}')
    # Append the times to the times.md file
    echo "| $i | $time1 | $time2 |" >> times.md
done


# Print the times as a table
cat times.md