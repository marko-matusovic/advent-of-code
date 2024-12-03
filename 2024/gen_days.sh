cd solutions

for d in {1..25}
do
    od=$(printf "%02d" $d)

    if [ ! -f "day$od.go" ]; then
        sed "s/XXX/$od/g" day.XXX > "day$od.go"
    fi
done
