# Setup:
you need to download and copy the mathjax to local:

```bash


cd assets/mathjax

curl -L https://github.com/mathjax/MathJax/archive/refs/tags/3.2.2.zip -o mathjax.zip
unzip mathjax.zip
mv MathJax-3.2.2/* .
rm -rf MathJax-3.2.2 mathjax.zip

```

## TODO:
- [ ] deletetion of cards and decks, with warning dialog
- [ ] saving zip files, just the same as images, works for desktop but mobile must be done