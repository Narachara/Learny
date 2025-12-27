# Setup:
you need to download and copy the mathjax to local:

```bash

cd myapp/assets
mkdir mathjax
cd mathjax
curl -L https://github.com/mathjax/MathJax/archive/refs/tags/3.2.2.zip -o mathjax.zip
unzip mathjax.zip
mv MathJax-3.2.2/* .
rm -rf MathJax-3.2.2 mathjax.zip

```

## TODO:
- [ ] Implement the deck export via the plugin and not in backend code
- [ ] Logo ändern
- [ ] Search function. Able to search text blocks. 
- [ ] Alle Debug statements entfernen