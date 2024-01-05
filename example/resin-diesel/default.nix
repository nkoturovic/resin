{
  system ? builtins.currentSystem,
  lock ? builtins.fromJSON (builtins.readFile ./flake.lock),
  appConfig ? builtins.fromJSON (builtins.readFile ./config.json),
  # The official nixpkgs input, pinned with the hash defined in the flake.lock file
  pkgs ? let
    nixpkgs = fetchTarball {
      url = "https://github.com/NixOS/nixpkgs/archive/${lock.nodes.nixpkgs.locked.rev}.tar.gz";
      sha256 = lock.nodes.nixpkgs.locked.narHash;
    };
  in
    import nixpkgs {
      overlays = [];
      config = {};
      inherit system;
    },
  # Custom nixpkgs channel, owner's nickname is kotur, hence kotur-nixpkgs
  kotur-nixpkgs ? let
    koturPkgs = fetchTarball {
      url = "https://github.com/nkoturovic/kotur-nixpkgs/archive/${lock.nodes.koturNixPkgs.locked.rev}.tar.gz";
      sha256 = lock.nodes.koturNixPkgs.locked.narHash;
    };
  in
    import koturPkgs {
      inherit system;
    },
}: let
  package = pkgs.rustPlatform.buildRustPackage rec {
    name = "resin";
    version = "0.0.5";
    cargoLock.lockFile = ./Cargo.lock;
    src = pkgs.lib.cleanSource ./.;

    # Programs and libraries used/available at build-time
    nativeBuildInputs = with pkgs; [
      ncurses

      cargo
      cargo-expand
      rustc
      diesel-cli

      glibcLocales
      postgresql

      openssl
    ];

    # src = builtins.path {
    #   path = ./.;

    #   # Filter all files that begin with '.', for example '.git', that way
    #   # .git directory will not become part of the source of our package
    #   filter = path: type:
    #     !(pkgs.lib.hasPrefix "." (baseNameOf path));
    # };

    passthru = {
      # inherit has nothing to do with OOP, it's a nix-specific syntax for
      # inheriting (copying) variables from the surrounding lexical scope
      inherit pkgs shell;
      # equivalent to:
      # pkgs = pkgs
      # shell = shell
    };
  };

  # Development shell
  shell = pkgs.mkShell {
    # Copy build inputs (dependencies) from the derivation the nix-shell environment
    # That way, there is no need for speciying dependenvies separately for derivation and shell
    inputsFrom = [
      package
    ];

    # Shell (dev environment) specific packages
    packages = with pkgs; [
      kotur-nixpkgs.dinosay # packet loads from the custom nixpkgs (kotur-nixpkgs)
    ];

    hardeningDisable = ["all"];

    LANG = "en_US.UTF-8";
    PGDATABASE = appConfig.database.name;
    PGDATA = toString ./.pg/pgdata;
    PGHOST = appConfig.database.host; # toString ./.pg/sockets;
    PGPORT = appConfig.database.port;
    PGPASS = appConfig.database.password;

    # Hook used for modifying the prompt look and printing the welcome message
    shellHook = ''
      export PGUSER="$USER"
      export DATABASE_URL="postgres://$PGUSER:$PGPASS@$PGHOST:$PGPORT/$POSTGRES_DB"
      PS1="\[\e[32m\][\[\e[m\]\[\e[33m\]nix-shell\\[\e[m\]:\[\e[36m\]\w\[\e[m\]\[\e[32m\]]\[\e[m\]\\$\[\e[m\] "
      alias ll="ls -l"
      dinosay -r -b happy -w 60 "Welcome to the '${package.name}' dev environment!"

      # set -v
      # set +v

      # Database
      # https://unix.stackexchange.com/questions/464106/killing-background-processes-started-in-nix-shell
      # # Create a database with the data stored in the current directory
      # initdb -D .pg/$PGDATABASE

      # # Start PostgreSQL running as the current user
      # # and with the Unix socket in the current directory
      # pg_ctl -D .pg/demodb -l log/pg.log -o "--unix_socket_directories='$PWD'" start
      #
      # # Create a database

      trap "'$PWD/.pg/client.sh' remove" EXIT
      .pg/client.sh add

      echo "ALTER USER $USER WITH PASSWORD '$PGPASS'" | psql

      # psql -lqt | cut -d \| -f 1 | grep -qw $PGDATABASE
      # if [ "$?" == "1" ]; then
      #   # db does not exist
      #   echo "Creating '$PGDATABASE'"
      #   createdb $PGDATABASE
      # fi
    '';
  };
in
  package
