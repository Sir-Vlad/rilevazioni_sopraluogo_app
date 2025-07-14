#!/bin/bash

set -e

echo "🚀 Setup ambiente di sviluppo PostgreSQL..."

# Controlla se Docker è installato
if ! command -v docker &> /dev/null; then
    echo "❌ Docker non trovato. Installa Docker prima di continuare."
    exit 1
fi

# Controlla se docker-compose è installato
if ! command -v docker compose &> /dev/null; then
    echo "❌ docker-compose non trovato. Installa docker-compose prima di continuare."
    exit 1
fi

# Crea file .env se non esiste
if [ ! -f .env ]; then
    echo "📝 Creazione file .env..."
    cp .env.example .env
    echo "✅ File .env creato da .env.example"
fi

# Avvia i container
echo "🐳 Avvio container PostgreSQL..."
docker compose up -d postgres

# Attendi che PostgreSQL sia pronto
echo "⏳ Attendo che PostgreSQL sia pronto..."
sleep 10

# Verifica connessione
echo "🔍 Verifica connessione al database..."
if docker compose exec postgres pg_isready -U app_user -d app_development > /dev/null 2>&1; then
    echo "✅ PostgreSQL è pronto!"
else
    echo "❌ Errore: PostgreSQL non risponde"
    exit 1
fi

# Installa Diesel CLI se non presente
if ! command -v diesel &> /dev/null; then
    echo "🔧 Installazione Diesel CLI..."
    cargo install diesel_cli --no-default-features --features postgres
    echo "✅ Diesel CLI installato"
fi

# Setup Diesel
echo "🔧 Setup Diesel..."
diesel setup

echo "🎉 Setup completato!"
echo ""
echo "Comandi utili:"
echo "  make db-up      # Avvia PostgreSQL"
echo "  make db-down    # Ferma PostgreSQL"
echo "  make db-shell   # Accedi alla shell PostgreSQL"
echo "  make db-logs    # Mostra i logs"
echo ""
echo "pgAdmin disponibile su: http://localhost:8080"
echo "  Email: admin@example.com"
echo "  Password: admin"