"""Main entry point for ingestion of anforanden https://data.riksdagen.se/data/anforanden/"""

import argparse
import sys

import pandas as pd
import numpy as np
import pathlib
import psycopg2

from tqdm import tqdm


PARTY_CONVERSION = {
    "FP": "L",
    "L": "L",
    "S": "S",
    "V": "V",
    "MP": "MP",
    "C": "C",
    "SD": "SD",
    "M": "M",
    "KD": "KD"
}

if __name__ == "__main__":
    argparser = argparse.ArgumentParser()
    argparser.add_argument("--data_dir",
                           required=True,
                           type=pathlib.Path,
                           help="path to data directory containing the csvt files")
    argparser.add_argument("--dbconnection",
                           type=str,
                           help="database connection string",
                           default="host=0.0.0.0 user=polidb password=develop")

    args = argparser.parse_args()

    conn = psycopg2.connect(args.dbconnection)

    cursor = conn.cursor()
    for file in args.data_dir.glob("*"):
        print(f"Processing {file}")
        f = pd.read_csv(file, error_bad_lines=False, warn_bad_lines=True)

        unknowns, unknown_parties = 0, set()
        for dokid, datum, rubrik, text, parti in tqdm(list(zip(f['dok_id'],
                                                               f['dok_datum'],
                                                               f['avsnittsrubrik'],
                                                               f['anforandetext'],
                                                               f['parti']))):
            #text = text.replace("'", "")

            parti = str(parti).upper()
            if parti in PARTY_CONVERSION:
                cursor.execute("INSERT INTO anforande (dokid, time, affiliation, content) VALUES (%s, %s, %s, %s)",
                               (dokid, datum, PARTY_CONVERSION[parti], text))
            else:
                unknowns += 1
                unknown_parties.add(parti[:5])

            #break

        print(f"{unknowns}/{len(f)} Dropped in {file} - Unknowns {unknown_parties}")

    conn.commit()

    cursor.close()
    conn.close()
