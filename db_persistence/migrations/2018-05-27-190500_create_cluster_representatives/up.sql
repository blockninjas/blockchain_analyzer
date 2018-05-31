CREATE TABLE cluster_representatives (
  address BIGINT PRIMARY KEY REFERENCES addresses (id),
  representative BIGINT NOT NULL REFERENCES addresses (id)
)
