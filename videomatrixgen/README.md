`video-matrix-gen` est un programme qui permet de générer une matrice de distance à partir d'une vidéo.

Fonctionnement :

- Le programme prend en entrée une vidéo.
- On recupère les frames de la vidéo à une fréquence d'une image par secondes. (ou plus si spécifié avec le flag --fps <fps>)
- Pour chaque frame, on calcule la différence normalisée entre la frame actuelle et la frame précédente.
- Il exporte le résultat sous forme d'une matrice de différence dans un fichier json compatible avec le programme `tcurve`.

Pré-Requis :

- ffmpeg (https://doc.ubuntu-fr.org/ffmpeg)

Usage: videomatrixgen [OPTIONS] <video_path>

Arguments:
<video_path> Chemin de la vidéo

Options:
-o, --output <output_path>  Chemin du fichier de sortie [par défaut: ./output.json]
    --fps <fps>             Fréquence d'échantillonnage des frames [par défaut: 1]
-h, --help                  Afficher l'aide
-V, --version               Afficher la version