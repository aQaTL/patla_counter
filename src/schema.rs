table! {
	counters (id) {
		id -> Int4,
		name -> Nullable<Text>,
	}
}

table! {
	entries (id) {
		id -> Int4,
		reason -> Nullable<Text>,
		created -> Timestamp,
		counter_id -> Int4,
	}
}

table! {
	passwords (id) {
		id -> Int4,
		password_hash -> Bpchar,
	}
}

joinable!(entries -> counters (counter_id));

allow_tables_to_appear_in_same_query!(counters, entries, passwords,);
