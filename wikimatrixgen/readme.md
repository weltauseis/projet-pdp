## wikimatrixgen

Programme en ligne de commande permettant de générer des fichiers de matrices de distance à partir d'articles wikipedia.

Se référer à l'option `--help` :

```
A simple tool to generate distance matrices for time curves visualisation from a wikipedia article.

Usage: wikimatrixgen [OPTIONS] <PAGE> <OUTPUT>

Arguments:
  <PAGE>    name of the wikipedia page in URL, e.g. "Hideo_Kojima"
  <OUTPUT>  output file

Options:
  -l, --lang-code <CODE>          language code of the wikipedia page : en, fr, de, ... [default: en]
  -n, --number <NUMBER>           Number of latest revisions to take into account [default: 20]
  -o, --older-than <REVISION_ID>  If specified, include only revisions older than this revision
      --legacy                    Save the timestamps in a format compatible with the original Java implementation
  -h, --help                      Print help
```
