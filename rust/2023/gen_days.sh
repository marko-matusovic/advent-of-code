cd src/solutions

for d in {1..25}
do
    od=$(printf "%02d" $d)

    if [ ! -f "day_$od.rs" ]; then
        sed "s/XXX/$od/g" day_template.rs > "day_$od.rs"
    fi
done
