echo "======== Example 0"
for i in {1..7}
do
    echo "2^${i} bits"
    ./coder ./testy4/example0.tga ./wyniki/example0_${i}.coded ${i}
    ./decoder ./wyniki/example0_${i}.coded ./wyniki/example0_${i}.png
    ./compare ./testy4/example0.tga ./wyniki/example0_${i}.png
done

echo "======== Example 1"
for i in {1..7}
do
    echo "2^${i} bits"
    ./coder ./testy4/example1.tga ./wyniki/example1_${i}.coded ${i}
    ./decoder ./wyniki/example1_${i}.coded ./wyniki/example1_${i}.png
    ./compare ./testy4/example1.tga ./wyniki/example1_${i}.png
done

echo "======== Example 2"
for i in {1..7}
do
    echo "2^${i} bits"
    ./coder ./testy4/example2.tga ./wyniki/example2_${i}.coded ${i}
    ./decoder ./wyniki/example2_${i}.coded ./wyniki/example2_${i}.png
    ./compare ./testy4/example2.tga ./wyniki/example2_${i}.png
done

echo "======== Example 3"
for i in {1..7}
do
    echo "2^${i} bits"
    ./coder ./testy4/example3.tga ./wyniki/example3_${i}.coded ${i}
    ./decoder ./wyniki/example3_${i}.coded ./wyniki/example3_${i}.png
    ./compare ./testy4/example3.tga ./wyniki/example3_${i}.png
done

