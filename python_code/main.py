import ast
import sys
import argparse
from my_lib.transform import transform_n_load



def handle_arguments(args):
    """add action based on inital calls"""
    parser = argparse.ArgumentParser(description="DE Transform Script")
    parser.add_argument(
        "Functions",
        choices=[
            "transform_n_load",
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
    else:
        print(f"Unknown function: {args.action}")


if __name__ == "__main__":
    main()
