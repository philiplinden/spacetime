jupyter nbconvert $1.ipynb --to html
cat $1.html > ../docs/examples/$1.md
rm $1.html
