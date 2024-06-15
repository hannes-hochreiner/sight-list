export def run [] {
  with-env {
    SIGHT_LIST_CONFIG: './config.json'
    ROCKET_PORT: 8100
    ROCKET_DATABASES: '{db={url="postgresql://sight_list:sight_list@localhost:8101"}}'
  } {
    ^cargo run
  }
}

export def create_db [] {
  ^docker run -d -p 8101:5432 --name sight_list-postgres -v ./db:/docker-entrypoint-initdb.d -e POSTGRES_PASSWORD=sight_list -e POSTGRES_DB=sight_list -e POSTGRES_USER=sight_list postgres:alpine
}

export def remove_db [] {
  ^docker rm sight_list-postgres
}

export def stop_db [] {
  ^docker stop sight_list-postgres
}

export def start_db [] {
  ^docker start sight_list-postgres
}

export def recreate_db [] {
  stop_db
  remove_db
  create_db
}

export def nix-build [] {
  ^nix build
}

export def update [] {
  ^cargo update
  ^nix flake update
}
