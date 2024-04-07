# Time Curves

## Organisation du projet :

Oubliez pas qu'on devra rendre ce repo au chercheur(s) qui nous corrigeront donc pas de commits bizarres et un workflow un minimum sérieux svp 🙏🙏🙏

🚨 Commitez absolument du code formatté (`shift + alt + f` sur vscode) pour éviter de devoir faire des commits de formatage qui polluent le repo 🚨

## Trucs à faire :

### Documentation :

Rajouter de la documentation pour TOUS les types et TOUTES les fonctions auxquelles les utilisateurs vont être exposés, comme j'ai commencé à faire dans input.rs :

```rust
/// Creates a new `InputData` object from a JSON string.
///
/// # Arguments
///
/// * `string` - A JSON string representing the input data.
///
/// # Returns
///
/// Returns a `Result` containing the parsed `InputData` object or an error if parsing fails.
pub fn from_str(string: &str) -> Result<Self, Box<dyn Error>> {
    let input: Self = serde_json::from_str(string)?;
    Ok(input)
}
```

Ce genre de commentaires sont gérés par cargo et permettent de génerer une documentation gratuitement avec `cargo doc --no-deps`.

### Amélioration du code :

Il y a plein de TODO dans le code, il suffit de chercher avec ctrl + shift + f, ça vaut le coup de regarder.

### Binding python :

Faire une branche et commencer à rendre la biblio utilisable en python, par exemple en convertissant le CLI en python.

### Tests :

Faire des tests pour les fonctions de la bibliothèque, je sais que vous adorez ça les gars en génie log.
