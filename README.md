# Setup:
you need to download and copy the mathjax to local:

```bash

cd myapp/assets
mkdir mathjax
curl -L https://github.com/mathjax/MathJax/archive/refs/tags/3.2.2.zip -o mathjax.zip
unzip mathjax.zip
mv MathJax-3.2.2/* .
rm -rf MathJax-3.2.2 mathjax.zip

```

## TODO:
- [ ] Block reordering implementieren
- [ ] Implement the deck export via the plugin and not in backend code
- [ ] Delete for blocks with files must delete the file when file gets replaced in edit mode
- [ ] Export funktion anpassen so dass Files und bilder auch exportiert werden
- [ ] Logo ändern
- [ ] Alle Debug statements entfernen
- [ ] Test the new branch on android !!!