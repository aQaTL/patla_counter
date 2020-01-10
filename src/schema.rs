table! {
	entries (id) {
		id -> Int4,
		reason -> Nullable<Text>,
		created -> Timestamp,
	}
}

table! {
	passwords (id) {
		id -> Int4,
		password_hash -> Bpchar,
	}
}

allow_tables_to_appear_in_same_query!(entries, passwords,);
