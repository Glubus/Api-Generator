# Osef.gg Backend

Backend pour la plateforme Osef.gg, un générateur de code intelligent qui s'adapte à vos langages et frameworks préférés.

## À propos du projet

Osef.gg est un générateur de code qui permet de créer rapidement des projets en fonction des langages de programmation et frameworks choisis. Le système distingue les frameworks client et serveur pour générer une architecture adaptée à chaque besoin.

## Technologies utilisées

- 🦀 Rust - Langage de programmation performant et sûr
- 🚀 Axum - Framework web moderne et asynchrone
- 🗄️ PostgreSQL - Base de données relationnelle robuste
- 📝 SQLx - Bibliothèque SQL type-safe pour Rust
- 📚 OpenAPI/Swagger - Documentation d'API interactive
- 🔄 Docker - Conteneurisation pour le développement et le déploiement

## Fonctionnalités

- ✅ Gestion des langages de programmation
- ✅ Gestion des frameworks (client/serveur)
- ✅ API RESTful pour accéder aux données
- ✅ Documentation OpenAPI intégrée
- ✅ Structure de base de données optimisée avec migrations

## Installation

### Prérequis

- Rust (dernière version stable)
- PostgreSQL
- Docker (optionnel, pour le développement)
- SQLx CLI (`cargo install sqlx-cli`)

### Étapes d'installation

1. Cloner le dépôt :
```bash
git clone https://github.com/votre-username/osef-gg-backend.git
cd osef-gg-backend
```

2. Configurer la base de données :
```bash
# Démarrer PostgreSQL avec Docker (optionnel)
docker-compose up -d

# Ou configurer votre propre instance PostgreSQL
# et modifier config.toml en conséquence
```

3. Exécuter les migrations :
```bash
sqlx migrate run
```

4. Compiler et lancer le serveur :
```bash
cargo run
```

Le serveur sera accessible à l'adresse `http://localhost:3000`.

## Structure du projet

```
.
├── migrations/        # Migrations SQLx pour la base de données
│   ├── 20250606220312_language.sql   # Création des tables de langages et frameworks
│   └── 20250606221953_fix_type_bug.sql  # Correction du champ type
├── src/
│   ├── handlers/      # Gestionnaires des requêtes HTTP
│   │   ├── framework.rs  # Endpoints pour les frameworks
│   │   └── language.rs   # Endpoints pour les langages
│   ├── models/        # Modèles de données
│   │   └── language.rs   # Structures pour langages et frameworks
│   ├── routes/        # Configuration des routes
│   ├── config.rs      # Configuration de l'application
│   ├── db.rs          # Gestion de la connexion à la base de données
│   ├── lib.rs         # Fonctions partagées
│   └── main.rs        # Point d'entrée de l'application
├── config.toml        # Fichier de configuration
└── Cargo.toml         # Dépendances du projet
```

## API Reference

### Langages de programmation

- `GET /api/language` - Liste tous les langages de programmation
- `GET /api/language/{id}` - Récupère un langage spécifique avec ses frameworks

### Frameworks

- `GET /api/framework` - Liste tous les frameworks avec leurs langages associés
- `GET /api/framework/{id}` - Récupère un framework spécifique avec son langage

## Base de données

Le schéma de la base de données comprend :

- Table `programming_languages` - Stocke les langages de programmation
- Table `frameworks` - Stocke les frameworks avec leur type (client/serveur) et une référence au langage

## Développement

### Tests

Pour exécuter les tests :

```bash
cargo test
```

### Documentation de l'API

La documentation Swagger est disponible à l'adresse : `http://localhost:3000/api/swagger`

## Contribuer

Les contributions sont les bienvenues ! N'hésitez pas à ouvrir une issue ou une pull request.

## Licence

Ce projet est sous licence [MIT](LICENSE). 