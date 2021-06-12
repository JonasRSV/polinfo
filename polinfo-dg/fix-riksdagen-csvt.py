"""Riksdagen csvt files are not compatible with the CSV format.

The reason being that they include " in the 'anforandetext' field. " is a reserved character so including it escapes
the 'field'.

This code does not fix all of the data since invalid characters are also included in others field. But this fixed
90% of the data.
 """
import argparse
import pathlib


def find_nth(haystack, needle, n):
    start = haystack.find(needle)
    while start >= 0 and n > 1:
        start = haystack.find(needle, start + len(needle))
        n -= 1
    return start


if __name__ == "__main__":
    argparser = argparse.ArgumentParser()
    argparser.add_argument("--data_dir",
                           required=True,
                           type=pathlib.Path,
                           help="path to data directory containing the csvt files")

    args = argparser.parse_args()

    for file in args.data_dir.glob("*"):
        print(f"Processing {file}")

        with open(file, "r") as f:
            contents = f.read()

            valid_contents = []
            lines = list(contents.splitlines())

            # Don't want to mess with the header (0)
            for i, line in enumerate(lines[1:]):
                text_start = find_nth(line, '"', 19)
                text_end = len(line) - find_nth(line[::-1], '"', 7) - 1

                without_reserved_characters = line[text_start + 1: text_end].replace('"', "'")

                valid_contents.append(line[:text_start + 1] + without_reserved_characters + line[text_end:])

            contents = lines[0].replace('"', '') + "\n" + "\n".join(valid_contents)

        with open(file, "w") as f:
            f.write(contents)
