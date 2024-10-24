# Étape 1 : Utiliser l'image de base Rust
FROM rust:latest as builder

# Créer un répertoire de travail
WORKDIR /usr/src/app

# Copier le fichier de configuration et les sources
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Compiler l'application
RUN cargo build --release

# Étape 2 : Préparer l'image finale
FROM debian:buster-slim

# Installer les dépendances nécessaires
RUN apt-get update && apt-get install -y libpq-dev

# Copier le binaire depuis l'image de construction
COPY --from=builder /usr/src/app/target/release/theproject /usr/local/bin/theproject

# Définir la commande par défaut
CMD ["theproject"]
