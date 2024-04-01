# Projet de programmation Timecurves

Contenu du dépôt :

## `tcurves`

Programme en ligne de commande qui permet de générer des timecurves à partir de fichiers de matrices de distances. Pour un exemple de fichier, voir https://aviz.fr/~bbach/timecurves/.

Les formats supportés pour l'instant sont :

- csv
- svg
- tikz
- vega-lite

Pour l'utilisation, se référer à l'option `--help` :

```
Usage: tcurves [OPTIONS] --format <FORMAT> <INPUT> <OUTPUT>

Arguments:
  <INPUT>   Specifies the input file for generating the curves. The file must be in the correct JSON format, as per the provided template
  <OUTPUT>  Specifies the output file for the generated curves. The file will be in the format specified by the --format option

Options:
  -f, --format <FORMAT>  Specifies the format of the output file
  -v, --verbose          Print additional debug information to the standard output
  -s, --size <SIZE>      Specifies the size of the output graph, for formats that support it. Unit is cm for Tikz, px for Vega-lite
  -h, --help             Print help
```

## timecurve-rs

Bibliothèque rust contenant tout le code de projection et de manipulation des timecurves. Le programme `tcurve` se contente principalement de faire des appels à cette bibliothèque.

La documentation n'est pas encore disponible.

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
  -h, --help                      Print help
```
