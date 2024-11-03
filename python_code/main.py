import ast
import sys
import argparse
from my_lib.transform import transform_n_load
from datetime import datetime
from my_lib.util import log_tests


def handle_arguments(args):
    """add action based on inital calls"""
    parser = argparse.ArgumentParser(description="DE Transform Script")
    parser.add_argument(
        "Functions",
        choices=[
            "transform_n_load",
            "speed_test"
        ],
    )

    args = parser.parse_args(args[:1])
    print(args.Functions)
    if args.Functions == "transform_n_load":
        parser.add_argument("local_dataset")
        parser.add_argument("database_name")
        parser.add_argument("new_data_tables")
        parser.add_argument("new_lookup_tables")
        parser.add_argument("column_attributes")
        parser.add_argument("column_map")
    elif args.Functions == "speed_test":
        parser.add_argument("new_data_tables")
        parser.add_argument("new_lookup_tables")
        parser.add_argument("column_attributes")
        parser.add_argument("column_map")

 
    # parse again
    return parser.parse_args(sys.argv[1:])


def main():
    """handles all the cli commands"""

    args = handle_arguments(sys.argv[1:])

    if args.Functions == "transform_n_load":
        print("Transforming and loading data...")
        print(
            transform_n_load(
                args.local_dataset,
                args.database_name,
                ast.literal_eval(args.new_data_tables),
                ast.literal_eval(args.new_lookup_tables),
                ast.literal_eval(args.column_attributes),
                ast.literal_eval(args.column_map),
            )
        )
    elif args.Functions == "speed_test":
        print("Starting python speed test...")
        log_tests("Python speed test started at server date and time: {}".format(datetime.now().strftime("%Y-%m-%d %H:%M:%S")), file_name="speed_test_data/Speed_Test_Result.md")

        start = datetime.now()
        transform_n_load(
            'air_quality.csv',
            'python_air_quality.db',
            ast.literal_eval(args.new_data_tables),
            ast.literal_eval(args.new_lookup_tables),
            ast.literal_eval(args.column_attributes),
            ast.literal_eval(args.column_map),
            )
        end = datetime.now()
                
        log_tests("The Python Speed test took: {} seconds to complete.".format((end - start).total_seconds()), file_name="speed_test_data/Speed_Test_Result.md")
        log_tests("Python speed test ended at server date and time: {}".format(datetime.now().strftime("%Y-%m-%d %H:%M:%S")), file_name="speed_test_data/Speed_Test_Result.md")
        log_tests("---------------------------------------------------------", file_name="speed_test_data/Speed_Test_Result.md")

        print(f"Python took: {(end - start).total_seconds()} seconds to complete the load and save operation.")
        print("End of python speed test. The result can be found in the test_speed folder.")

    else:
        print(f"Unknown function: {args.action}")


if __name__ == "__main__":
    main()
