install:
	pip install --upgrade pip && pip install -r python_code/requirements.txt

format:
	black python_code/*.py

lint:
	ruff check python_code/*.py python_code/my_lib/*.py

test:
	python -m pytest -cov=main python_code/test_main.py

test_speed:
	python python_code/main.py speed_test '{"air_quality":["air_quality_id","fn_indicator_id","fn_geo_id","time_period","start_date","data_value"]}' '{"indicator":["indicator_id","indicator_name","measure","measure_info"],"geo_data":["geo_id","geo_place_name","geo_type_name"]}' '{"air_quality_id":"INTEGER PRIMARY KEY","indicator_id":"INTEGER PRIMARY KEY","indicator_name":"TEXT","measure":"TEXT","measure_info":"TEXT","geo_type_name":"TEXT","geo_id":"INTEGER PRIMARY KEY","geo_place_name":"TEXT","time_period":"TEXT","start_date":"TEXT","data_value":"REAL","fn_indicator_id":"INTEGER","fn_geo_id":"INTEGER"}' '{"air_quality_id":0,"indicator_id":1,"indicator_name":2,"measure":3,"measure_info":4,"geo_type_name":5,"geo_id":6,"geo_place_name":7,"time_period":8,"start_date":9,"data_value":10,"fn_geo_id":6,"fn_indicator_id":1}'

all: install format lint test test_speed query


  