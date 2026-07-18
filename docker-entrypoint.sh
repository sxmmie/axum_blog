#!/bin/sh
set -eu

if [ "$1" = "migrate" ]; then
  echo "Running database migrations..."
  until pg_isready -h db -U user -d axum_lite; do
    echo "Waiting for PostgreSQL..."
    sleep 2
  done

  export PGPASSWORD="${PGPASSWORD:-password}"
  export DATABASE_URL="${DATABASE_URL:-postgres://user:password@db:5432/axum_lite}"

  for sql_file in $(find /app/migrations -maxdepth 1 -type f -name '*.up.sql' | sort); do
    echo "Applying $(basename "$sql_file")"
    psql "$DATABASE_URL" -v ON_ERROR_STOP=1 -f "$sql_file"
  done

  echo "Migrations complete"
  exit 0
fi

if [ "$1" = "app" ]; then
  echo "Starting application..."
  exec /app/axum_blog
fi

echo "Usage: docker-entrypoint.sh [migrate|app]"
exit 1
