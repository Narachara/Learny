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
- [ ] Rename Deck function und Deck options besser stylen
- [ ] Implement the deck export via the plugin and not in backend code
- [ ] Export funktion anpassen so dass Files und bilder auch exportiert werden
- [ ] Logo ändern
- [ ] Test on Android. Why no images are shown ? 
- [ ] Update versions and move back to main branch
- [ ] Alle Debug statements entfernen