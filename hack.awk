BEGIN { FS="=" }
/nb_frames/ { F = $2 }
/duration/ { D = $2 }
END {
	if (D / 60 >= 1)
		printf "%d\t%dmin\\&%02ds", F, D / 60, D % 60;
	else
		printf "%d\t\\&%02ds", F, D % 60
}
