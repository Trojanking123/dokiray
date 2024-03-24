set positional-arguments

startdb:
    sudo /etc/init.d/postgresql start || sudo service postgresql start

build:
    cargo build 

run:
    cargo run

@commit *args='':
    git add .
    git commit -m "$1"
    proxychains git push
    
fresh:
    DATABASE_URL="postgresql://root:123456@127.0.0.1:5432/dokiray"  sea-orm-cli migrate fresh -d ./dokiray-migration

refresh:
    DATABASE_URL="postgresql://root:123456@localhost:5432/dokiray"  sea-orm-cli migrate refresh -d ./dokiray-migration

down:
    DATABASE_URL="postgresql://root:123456@localhost:5432/dokiray"  sea-orm-cli migrate down -d ./dokiray-migration

pg:
    PGPASSWORD='123456' psql -d dokiray -U root -h 127.0.0.1

gen:
    DATABASE_URL="postgresql://root:123456@localhost:5432/dokiray"   sea-orm-cli generate entity  --output-dir ./dokiray-entity/src/ -l --with-serde both 

fmt:
    @cargo fmt

lint:
    @ cargo clippy --fix --allow-dirty