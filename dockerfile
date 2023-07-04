# Utilisez une image de base avec Rust préinstallé
FROM rust:latest

# Copiez le contenu de votre projet dans le conteneur Docker
COPY . /app

# Définissez le répertoire de travail
WORKDIR /app

# Exécutez les commandes de construction du projet
RUN cargo build --release

# Définissez la commande par défaut à exécuter lorsque le conteneur démarre
CMD ["cargo", "run", "--release"]