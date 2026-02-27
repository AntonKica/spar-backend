dev-deploy:
	ssh spar@chodiacidotaznik.xyz -t 'cd spar-backend; git pull; cargo build --release && systemctl --user restart spar-backend.service'
dev-setup:
	ssh spar@chodiacidotaznik.xyz -t 'source .bashrc; cd spar-backend; sqlx migrate revert; sqlx migrate run; cargo test --package spar-backend --test fill_database create_assets -- --exact'

database-run:
	cargo sqlx migrate run

database-reveret:
	cargo sqlx migrate revert
