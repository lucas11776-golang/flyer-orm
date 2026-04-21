



# Install Package From Github

cargo install --git https://github.com --branch dev


cargo install --git https://github.com/lucas11776-golang/sqlx --branch arguments --all-bins



cargo install --git https://github.com/lucas11776-golang/sqlx --branch arguments sqlx-cli --features postgres,mysql,sqlite




<!-- cargo add --git https://github.com/lucas11776-golang/sqlx --branch arguments -->


cargo add --git https://github.com/lucas11776-golang/sqlx --branch arguments 



cargo add --git https://github.com/lucas11776-golang/sqlx --branch arguments sqlx-sqlite sqlx-postgres sqlx-mysql sqlx-macros-core sqlx-macros sqlx-core uuid


cargo add --git https://github.com/lucas11776-golang/sqlx --branch arguments runtime-tokio-native-tls



sqlx = { git = "https://github.com/lucas11776-golang/sqlx", branch = "arguments", version = "0.8.6",features = ["any", "sqlite", "mysql", "postgres", "macros", "runtime-tokio-native-tls", "uuid"] }


sqlx = { git = "https://github.com/lucas11776-golang/sqlx", branch = "arguments", version = "0.9.0-alpha.1", features = ["any", "sqlite", "mysql", "postgres", "macros", "runtime-tokio-native-tls", "uuid"] }

