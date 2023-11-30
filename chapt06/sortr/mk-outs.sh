#!/usr/bin/env bash

ROOT="tests/inputs"
OUT_DIR="tests/expected"

[[ ! -d "$OUT_DIR" ]] && mkdir -p "$OUT_DIR"

# Cf https://github.com/coreutils/coreutils/blob/master/tests/misc/uniq.pl
echo -ne "1\n1\n"    > $ROOT/t1.txt
echo -ne "1\n1"      > $ROOT/t2.txt
echo -ne "1\n2"      > $ROOT/t3.txt
echo -ne "1\n1\n2"   > $ROOT/t4.txt
echo -ne "2\n1\n1\n" > $ROOT/t5.txt
echo -ne "1\n2\n3\n" > $ROOT/t6.txt

for FILE in $ROOT/*.txt; do
    BASENAME=$(basename "$FILE")
    sort      $FILE > ${OUT_DIR}/${BASENAME}.out
    #uniq -c   $FILE > ${OUT_DIR}/${BASENAME}.c.out
    sort    < $FILE > ${OUT_DIR}/${BASENAME}.stdin.out
    #uniq -c < $FILE > ${OUT_DIR}/${BASENAME}.stdin.c.out
done
