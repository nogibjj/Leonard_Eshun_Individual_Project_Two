from my_lib.transform import transform_n_load
from my_lib.util import log_tests


column_map = {
    "air_quality_id": 0,
    "indicator_id": 1,
    "indicator_name": 2,
    "measure": 3,
    "measure_info": 4,
    "geo_type_name": 5,
    "geo_id": 6,
    "geo_place_name": 7,
    "time_period": 8,
    "start_date": 9,
    "data_value": 10,
    "fn_geo_id": 6,
    "fn_indicator_id": 1,
}



# Test transform and load
def test_transform_and_load():
    log_tests("Transform and Load Test", header=True)
    transform_n_load(
        local_dataset="air_quality.csv",
        database_name="python_air_quality.db",
        new_data_tables={
            "air_quality": [
                "air_quality_id",
                "fn_indicator_id",
                "fn_geo_id",
                "time_period",
                "start_date",
                "data_value",
            ],
        },
        new_lookup_tables={
            "indicator": ["indicator_id", "indicator_name", "measure", "measure_info"],
            "geo_data": ["geo_id", "geo_place_name", "geo_type_name"],
        },
        column_attributes={
            "air_quality_id": "INTEGER PRIMARY KEY",
            "indicator_id": "INTEGER PRIMARY KEY",
            "indicator_name": "TEXT",
            "measure": "TEXT",
            "measure_info": "TEXT",
            "geo_type_name": "TEXT",
            "geo_id": "INTEGER PRIMARY KEY",
            "geo_place_name": "TEXT",
            "time_period": "TEXT",
            "start_date": "TEXT",
            "data_value": "REAL",
            "fn_indicator_id": "INTEGER",
            "fn_geo_id": "INTEGER",
        },
        column_map={
            "air_quality_id": 0,
            "indicator_id": 1,
            "indicator_name": 2,
            "measure": 3,
            "measure_info": 4,
            "geo_type_name": 5,
            "geo_id": 6,
            "geo_place_name": 7,
            "time_period": 8,
            "start_date": 9,
            "data_value": 10,
            "fn_geo_id": 6,
            "fn_indicator_id": 1,
        },
    )
    log_tests("Transform and Load Test Successful", last_in_group=True)
    print("Transform and Load Test Successful")


if __name__ == "__main__":
    test_transform_and_load()
 