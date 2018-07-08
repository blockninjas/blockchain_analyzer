# TODO Rename AWS docker repository
docker build -t bitcoin/db_importer -f Dockerfile.blockchain_analyzer .

$(aws ecr get-login --no-include-email --region eu-west-1) && \
  docker tag bitcoin/db_importer:latest 554480245419.dkr.ecr.eu-west-1.amazonaws.com/bitcoin/db_importer:latest && \
  docker push 554480245419.dkr.ecr.eu-west-1.amazonaws.com/bitcoin/db_importer:latest
