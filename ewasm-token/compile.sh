#!/bin/bash


EWASM_NAME=ewasm_token
TNAME=wrc20ChallengeFiller.yml

make

binary2hex $EWASM_NAME.wasm > $EWASM_NAME.hex
cat $EWASM_NAME.wat > watcopy

sed -i -e 's/^/        /g' watcopy

cat header.txt > $TNAME
echo "      code: '0x$(cat $EWASM_NAME.hex)'" >> $TNAME
cat footer.txt >> $TNAME

sed -i -e 's/"env"/"ethereum"/g' $TNAME
sed -i -e 's/"ethereum_/"/g'     $TNAME

cp -f $TNAME ~/workspace/tests2/src/GeneralStateTestsFiller/stEWASMTests/$TNAME
