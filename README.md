## Diceware Generator in RUST

##### All Credentials Are Beschützt

Lange, komplizierte Passwörter sind schwer zu merken und verleiten deshalb zu schlechten Praktiken. Eine Alternative bilden Passphrasen. Mehr dem Thema [hier](https://de.wikipedia.org/wiki/Diceware). Dieses Applet generiert Passphrasen in Längen von sechs bis 26 Worten rein zufällig.

![xkcd comic zum thema diceware](https://imgs.xkcd.com/comics/password_strength.png)
### Installation:

Das Applet ist für verschiedene Targets compiliert worden. Die Binaries sind [hier]() zu finden.

Alternativ kann mensch auch selbst compilieren. Dazu wird die [Rust Umgebung](https://rustup.rs/) benötigt. Anschließend reichen die folgenden Befehle:


```
git clone https://github.com/ruebe1312/diceware_generator.git
cd diceware_generator
cargo run
```

Dabei sollte die Datei `diceware.txt` im root Verzeichnis liegen. Wird eine bereits compilierte Binary benutzt muss ich die Textdatei im selben Verzeichnis, wie die Binary befinden.
### diceware.txt

Die verwendete Wortliste ist aus dem Repo [https://github.com/dys2p/wordlists-dev](https://github.com/dys2p/wordlists-de). Vielen Dank für das zusammenstellen!