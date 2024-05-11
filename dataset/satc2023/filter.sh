for f in ./*
do
    bt=`wc $f | awk '{print $3}'`
    if [ "$bt" -gt 30000000 ]; then
        echo $f
        rm $f
    fi
done

echo > ../satc2023.txt
for f in ./*.cnf
do
    echo satc2023/$f >> ../satc2023.txt
done
