for i in {1..10}
do
    ./CreateSAT -g u -v 10 -k 3 -c 10 -s $RANDOM -f random/random$i
done

for i in {11..20}
do
    ./CreateSAT -g u -v 15 -k 4 -c 10 -s $RANDOM -f random/random$i
done

for i in {21..30}
do
    ./CreateSAT -g u -v 20 -k 5 -c 10 -s $RANDOM -f random/random$i
done

for i in {31..40}
do
    ./CreateSAT -g u -v 20 -k 5 -c 15 -s $RANDOM -f random/random$i
done

for i in {41..50}
do
    ./CreateSAT -g u -v 15 -k 4 -c 5 -s $RANDOM -f random/random$i
done

for i in {51..60}
do
    ./CreateSAT -g u -v 10 -k 5 -c 15 -s $RANDOM -f random/random$i
done

for i in {61..70}
do
    ./CreateSAT -g u -v 5 -k 3 -c 10 -s $RANDOM -f random/random$i
done

for i in {71..80}
do
    ./CreateSAT -g u -v 10 -k 3 -c 20 -s $RANDOM -f random/random$i
done

for i in {81..90}
do
    ./CreateSAT -g u -v 20 -k 10 -c 15 -s $RANDOM -f random/random$i
done

for i in {91..100}
do
    ./CreateSAT -g u -v 10 -k 5 -c 10 -s $RANDOM -f random/random$i
done
