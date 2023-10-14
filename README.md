Frontend:
fly deploy --build-only
fly deploy

Backend:
fly deploy --build-only --build-arg "PROD_DB=$PROD_DB"
fly deploy --build-arg "PROD_DB=$PROD_DB" --env "PORT=8080"


