INSERT OR REPLACE INTO entries (
	type_,
	date_string,
	date_,
	sgv,
	direction,
	noise,
	filtered,
	unfiltered,
	rssi
) VALUES (
	:type_,
	:date_string,
	:date_,
	:sgv,
	:direction,
	:noise,
	:filtered,
	:unfiltered,
	:rssi
)
