Programme en ligne de commande qui permet de générer des timecurves à partir de fichiers de matrices de distances. Pour un exemple de fichier, voir https://aviz.fr/~bbach/timecurves/.

Les formats supportés pour l'instant sont :

    csv
    svg
    tikz
    vega-lite

Pour l'utilisation, se référer à l'option --help :

```
Usage: tcurves [OPTIONS] --format <FORMAT> <INPUT> <OUTPUT>

Arguments:
  <INPUT>   Specifies the input file for generating the curves. The file must be in the correct JSON format, as per the provided template
  <OUTPUT>  Specifies the output file for the generated curves. The file will be in the format specified by the --format option

Options:
  -f, --format <FORMAT>        Specifies the format of the output file
  -s, --size <SIZE>            Specifies the size of the output graph, for formats that support it. Unit is cm for Tikz, px for Vega-lite
      --thickness <THICKNESS>  Specifies the thickness of the lines in the output graph, for formats that support it [default: 1.0]
  -h, --help                   Print help
```

L'outil implémente des logs de débugage via la variable d'environnement RUST_LOG, voir https://docs.rs/env_logger/.

Le dossier `data/` comporte les datasets d'exemple.
