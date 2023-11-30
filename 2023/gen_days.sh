cd src/solutions

for d in {1..24}
do
    od=$(printf "%02d" $d)

    rm day_$od.rs

    sed "s/xxx/$d/g" day_template.rs > "day_$od.rs"
done
