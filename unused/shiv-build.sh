#!/usr/bin/env bash

rm -r dist/* prtest.pyz
# python3 = /usr/bin/env python3
python3 setup.py sdist

cp -r \
-t dist \

echo
echo Installing local app
pip3 install . --target dist

echo
echo "Starting shiv"

# shiv --site-packages dist --compressed -p '/usr/bin/env python' -o csv.pyz -e prtest.main:main
shiv --site-packages dist --compressed -p '/usr/bin/python3' -o prtest.py -e prtest.main:main \
    --root ".shiv/"
    # "/usr/local/www/bert/pbrooks/spring2021/temp"
# shiv --site-packages dist --compressed -o prtest.pyz -e prtest.main:main

echo done