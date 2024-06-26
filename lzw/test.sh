echo "======== Delta 1"
./lzw_coder delta ./testy/test1.bin ./wyniki/test1.comp
./lzw_decoder delta ./wyniki/test1.comp ./wyniki/test1.decomp
diff ./testy/test1.bin ./wyniki/test1.decomp && echo ok

echo "======== Gamma 1"
./lzw_coder gamma ./testy/test1.bin ./wyniki/test1.comp
./lzw_decoder gamma ./wyniki/test1.comp ./wyniki/test1.decomp
diff ./testy/test1.bin ./wyniki/test1.decomp && echo ok

echo "======== Omega 1"
./lzw_coder omega ./testy/test1.bin ./wyniki/test1.comp
./lzw_decoder omega ./wyniki/test1.comp ./wyniki/test1.decomp
diff ./testy/test1.bin ./wyniki/test1.decomp && echo ok

echo "======== Fib 1"
./lzw_coder fib ./testy/test1.bin ./wyniki/test1.comp
./lzw_decoder fib ./wyniki/test1.comp ./wyniki/test1.decomp
diff ./testy/test1.bin ./wyniki/test1.decomp && echo ok

echo "======== Delta 2"
./lzw_coder delta ./testy/test2.bin ./wyniki/test2.comp
./lzw_decoder delta ./wyniki/test2.comp ./wyniki/test2.decomp
diff ./testy/test2.bin ./wyniki/test2.decomp && echo ok

echo "======== Gamma 2"
./lzw_coder gamma ./testy/test2.bin ./wyniki/test2.comp
./lzw_decoder gamma ./wyniki/test2.comp ./wyniki/test2.decomp
diff ./testy/test2.bin ./wyniki/test2.decomp && echo ok

echo "======== Omega 2"
./lzw_coder omega ./testy/test2.bin ./wyniki/test2.comp
./lzw_decoder omega ./wyniki/test2.comp ./wyniki/test2.decomp
diff ./testy/test2.bin ./wyniki/test2.decomp && echo ok

echo "======== Fib 2"
./lzw_coder fib ./testy/test2.bin ./wyniki/test2.comp
./lzw_decoder fib ./wyniki/test2.comp ./wyniki/test2.decomp
diff ./testy/test2.bin ./wyniki/test2.decomp && echo ok

echo "======== Delta 3"
./lzw_coder delta ./testy/test3.bin ./wyniki/test3.comp
./lzw_decoder delta ./wyniki/test3.comp ./wyniki/test3.decomp
diff ./testy/test3.bin ./wyniki/test3.decomp && echo ok

echo "======== Gamma 3"
./lzw_coder gamma ./testy/test3.bin ./wyniki/test3.comp
./lzw_decoder gamma ./wyniki/test3.comp ./wyniki/test3.decomp
diff ./testy/test3.bin ./wyniki/test3.decomp && echo ok

echo "======== Omega 3"
./lzw_coder omega ./testy/test3.bin ./wyniki/test3.comp
./lzw_decoder omega ./wyniki/test3.comp ./wyniki/test3.decomp
diff ./testy/test3.bin ./wyniki/test3.decomp && echo ok

echo "======== Fib 3"
./lzw_coder fib ./testy/test3.bin ./wyniki/test3.comp
./lzw_decoder fib ./wyniki/test3.comp ./wyniki/test3.decomp
diff ./testy/test3.bin ./wyniki/test3.decomp && echo ok

echo "======== Delta Tadeusz"
./lzw_coder delta ./testy/pan_tadeusz.txt ./wyniki/pan_tadeusz.comp
./lzw_decoder delta ./wyniki/pan_tadeusz.comp ./wyniki/pan_tadeusz.decomp
diff ./testy/pan_tadeusz.txt ./wyniki/pan_tadeusz.decomp && echo ok

echo "======== Gamma Taduesz"
./lzw_coder gamma ./testy/pan_tadeusz.txt ./wyniki/pan_tadeusz.comp
./lzw_decoder gamma ./wyniki/pan_tadeusz.comp ./wyniki/pan_tadeusz.decomp
diff ./testy/pan_tadeusz.txt ./wyniki/pan_tadeusz.decomp && echo ok

echo "======== Omega Taduesz"
./lzw_coder omega ./testy/pan_tadeusz.txt ./wyniki/pan_tadeusz.comp
./lzw_decoder omega ./wyniki/pan_tadeusz.comp ./wyniki/pan_tadeusz.decomp
diff ./testy/pan_tadeusz.txt ./wyniki/pan_tadeusz.decomp && echo ok

echo "======== Fib Taduesz"
./lzw_coder fib ./testy/pan_tadeusz.txt ./wyniki/pan_tadeusz.comp
./lzw_decoder fib ./wyniki/pan_tadeusz.comp ./wyniki/pan_tadeusz.decomp
diff ./testy/pan_tadeusz.txt ./wyniki/pan_tadeusz.decomp && echo ok