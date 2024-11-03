"""
Transform the extracted data 
"""

import sqlite3
import csv
from my_lib.util import log_tests, db_path


def create_table(cursor, table, columns, column_attributes):
    cursor.execute(f"DROP TABLE IF EXISTS {table}")
    col_attrib_list = [(f"{col} {column_attributes[col]}") for col in columns]
    cursor.execute(f"CREATE TABLE {table} ({', '.join(col_attrib_list)})")


# load the csv file and insert into a new sqlite3 database
def transform_n_load(
    local_dataset: str,
    database_name: str,
    new_data_tables: dict,
    new_lookup_tables: dict,
    column_attributes: dict,
    column_map: dict,
):
    """ "Transforms and Loads data into the local SQLite3 database"""

    # load the data from the csv

    reader = csv.reader(open(db_path + "speed_test_data/" + local_dataset, newline=""), delimiter=",")

    conn = sqlite3.connect(db_path + "speed_test_data/" + database_name)

    c = conn.cursor()
    # Create tables
    for k, v in new_data_tables.items():
        log_tests(f"Creating non-lookup table: {k}")
        create_table(c, k, v, column_attributes)

    for k, v in new_lookup_tables.items():
        log_tests(f"Creating lookup table: {k}")
        create_table(c, k, v, column_attributes)
    log_tests("Tables created.")

    # skip the first row
    log_tests("Skipping the first row...")
    next(reader)
    log_tests("Inserting table data...")
    for row in reader:
        first_for_loop_broken = False
        for k, v in new_lookup_tables.items():
            # If the ID is not a number don't import it
            if not row[column_map[v[0]]].isnumeric():
                first_for_loop_broken = True
                break  # Go to outer loop

            exec_str = f"select count({v[0]}) from {k} where {v[0]} = {int(row[column_map[v[0]]])}"
            result = c.execute(exec_str).fetchone()[0]
            if result == 0:
                data_values = "', '".join([(row[column_map[col]]) for col in v])
                c.execute(f"INSERT INTO {k} ({', '.join(v)}) VALUES ('{data_values}')")
                conn.commit()

        # Only load the data if all lookup information are there
        if not first_for_loop_broken:
            for k, v in new_data_tables.items():
                data_values = "', '".join([(row[column_map[col]]) for col in v])
                c.execute(f"INSERT INTO {k} ({', '.join(v)}) VALUES ('{data_values}')")
            conn.commit()
    log_tests("Inserting table data completed")

    conn.close()

    return "Transform  and load Successful"
