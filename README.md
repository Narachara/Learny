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
- [ ] deletetion of decks, with warning dialog
- [ ] Block reordering implementieren
- [ ] Styling
- [ ] Implement the deck export via the plugin and not in backend code
- [ ] Score function und progress bar
- [ ] Export funktion anpassen so dass Files und bilder auch exportiert werden
- [ ] Logo und Titel ändern
- [ ] Alle Debug statements entfernen