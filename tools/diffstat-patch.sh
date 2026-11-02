#!/usr/bin/env bash
set -euo pipefail

usage() {
	printf 'Usage: %s -f PATCH [--stat]\n' "${0##*/}"
	printf '       %s -f - [--stat] < PATCH\n' "${0##*/}"
}

patch_file=""
stat_mode=0
while (($#)); do
	case "$1" in
		-f|--file)
			if (($# < 2)); then
				printf '%s: -f requires a patch path or -\n' "${0##*/}" >&2
				exit 2
			fi
			patch_file="$2"
			shift 2
			;;
		--stat)
			stat_mode=1
			shift
			;;
		-h|--help)
			usage
			exit 0
			;;
		*)
			printf '%s: unexpected argument: %s\n' "${0##*/}" "$1" >&2
			usage >&2
			exit 2
			;;
	esac
done

if [[ -z "$patch_file" ]]; then
	printf '%s: -f is required\n' "${0##*/}" >&2
	usage >&2
	exit 2
fi

if [[ "$patch_file" == "-" ]]; then
	exec 3<&0
elif [[ -r "$patch_file" && ! -d "$patch_file" ]]; then
	if ! exec 3< "$patch_file"; then
		printf '%s: cannot read patch: %s\n' "${0##*/}" "$patch_file" >&2
		exit 2
	fi
else
	printf '%s: cannot read patch: %s\n' "${0##*/}" "$patch_file" >&2
	exit 2
fi

awk -v stat_mode="$stat_mode" '
function add_file(filename,    row) {
	row = ++file_count
	files[row] = filename
	additions[row] = 0
	deletions[row] = 0
	binary[row] = 0
	return row
}

function header_path(line,    path, tab) {
	path = substr(line, 5)
	tab = index(path, "\t")
	if (tab) {
		path = substr(path, 1, tab - 1)
	}
	return path
}

function git_new_path(line,    path, marker, position) {
	path = substr(line, 12)
	marker = 0
	while ((position = index(substr(path, marker + 1), " b/")) > 0) {
		marker += position
	}
	if (marker) {
		path = substr(path, marker + 3)
	} else {
		path = ""
	}
	sub(/"$/, "", path)
	sub(/^"/, "", path)
	return path
}

function display_git_path(path) {
	sub(/^b\//, "", path)
	return path
}

function set_hunk_counts(line,    ranges, old_range, new_range, counts) {
	split(line, ranges, " ")
	old_range = substr(ranges[2], 2)
	new_range = substr(ranges[3], 2)
	split(old_range, counts, ",")
	old_remaining = (counts[2] == "" ? 1 : counts[2] + 0)
	split(new_range, counts, ",")
	new_remaining = (counts[2] == "" ? 1 : counts[2] + 0)
	in_hunk = (old_remaining > 0 || new_remaining > 0)
}

BEGIN {
	file_count = 0
	current = 0
	in_hunk = 0
	git_header_pending = 0
}

{
	if (in_hunk) {
		marker = substr($0, 1, 1)
		if (marker == "+") {
			additions[current]++
			new_remaining--
		} else if (marker == "-") {
			deletions[current]++
			old_remaining--
		} else if (marker == " ") {
			old_remaining--
			new_remaining--
		} else if (marker == "\\") {
			next
		} else {
			in_hunk = 0
		}

		if (marker == "+" || marker == "-" || marker == " ") {
			if (old_remaining <= 0 && new_remaining <= 0) {
				in_hunk = 0
			}
			next
		}
	}

	if (substr($0, 1, 11) == "diff --git ") {
		filename = display_git_path(git_new_path($0))
		current = add_file(filename)
		git_header_pending = 1
		old_path = ""
		in_hunk = 0
		next
	}

	if (substr($0, 1, 4) == "--- ") {
		old_path = header_path($0)
		in_hunk = 0
		next
	}

	if (substr($0, 1, 4) == "+++ " && old_path != "") {
		new_path = header_path($0)
		filename = (new_path == "/dev/null" ? old_path : new_path)
		if (git_header_pending && current) {
			files[current] = display_git_path(filename)
		} else {
			current = add_file(filename)
		}
		git_header_pending = 0
		old_path = ""
		next
	}

	if ($0 ~ /^Binary files .* and .* differ$/) {
		binary_text = $0
		sub(/^Binary files /, "", binary_text)
		sub(/ differ$/, "", binary_text)
		marker = 0
		while ((position = index(substr(binary_text, marker + 1), " and ")) > 0) {
			marker += position
		}
		binary_filename = (marker ? substr(binary_text, marker + 5) : binary_text)
		if (git_header_pending && current) {
			binary[current] = 1
		} else if (current && files[current] == binary_filename &&
				   additions[current] == 0 && deletions[current] == 0) {
			binary[current] = 1
		} else {
			current = add_file(binary_filename)
			binary[current] = 1
		}
		git_header_pending = 0
		in_hunk = 0
		next
	}

	if ($0 == "GIT binary patch" && current) {
		binary[current] = 1
		git_header_pending = 0
		in_hunk = 0
		next
	}

	if ($0 ~ /^@@ -[0-9]+(,[0-9]+)? \+[0-9]+(,[0-9]+)? @@/) {
		if (!current && old_path != "") {
			current = add_file(old_path)
		}
		if (current) {
			set_hunk_counts($0)
		}
	}
}

END {
	if (!file_count) {
		print "No file changes found."
		exit
	}

	total_additions = 0
	total_deletions = 0
	path_width = 0
	max_changes = 0
	count_width = 1
	for (row = 1; row <= file_count; row++) {
		total_additions += additions[row]
		total_deletions += deletions[row]
		changed[row] = additions[row] + deletions[row]
		if (length(files[row]) > path_width) {
			path_width = length(files[row])
		}
		if (changed[row] > max_changes) {
			max_changes = changed[row]
		}
		count_text = (binary[row] ? "Bin" : changed[row])
		if (length(count_text) > count_width) {
			count_width = length(count_text)
		}
	}

	if (stat_mode) {
		for (row = 1; row <= file_count; row++) {
			count_text = (binary[row] ? "Bin" : changed[row])
			printf "%-*s | %*s", path_width, files[row], count_width, count_text
			if (changed[row] > 0) {
				bar_length = int(changed[row] * 50 / max_changes + 0.5)
				if (bar_length < 1) {
					bar_length = 1
				}
				if (additions[row] && deletions[row] && bar_length < 2) {
					bar_length = 2
				}
				plus_count = int(bar_length * additions[row] / changed[row] + 0.5)
				if (additions[row] && plus_count < 1) {
					plus_count = 1
				}
				if (deletions[row] && plus_count >= bar_length) {
					plus_count = bar_length - 1
				}
				printf " "
				for (bar_index = 0; bar_index < plus_count; bar_index++) {
					printf "+"
				}
				for (bar_index = plus_count; bar_index < bar_length; bar_index++) {
					printf "-"
				}
			}
			printf "\n"
		}
	} else {
		width = (path_width > length("File") ? path_width : length("File"))
		printf "%-*s  %10s  %9s\n", width, "File", "Insertions", "Deletions"
		for (row = 1; row <= file_count; row++) {
			add_text = (binary[row] ? "binary" : additions[row])
			delete_text = (binary[row] ? "binary" : deletions[row])
			printf "%-*s  %10s  %9s\n", width, files[row], add_text, delete_text
		}
	}

	noun = (stat_mode && file_count == 1 ? "file" : "files")
	printf "%d %s changed, %d insertions(+), %d deletions(-)\n",
		file_count, noun, total_additions, total_deletions
}
' <&3