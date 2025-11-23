database-run:
	cargo sqlx migrate run

database-reveret:
	cargo sqlx migrate revert
