export DATABASE_URL=postgres://postgres:postgres@localhost:5432/postgres # local database
export API_PORT=8080
alias unit_test="docker compose -f docker-compose.test.yml up --build"
alias integration_test_build="docker compose -f docker-compose.integration.yml build \
                              --no-cache \
                              --build-arg PROD_DB=$PROD_DB"
alias integration_test_run="docker compose -f docker-compose.integration.yml up \
                              --abort-on-container-exit \
                              --exit-code-from frontend"
alias integration_test="integration_test_build && integration_test_run"
