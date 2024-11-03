# Data Engineering Individual Project Two 
[![Rust CI/CD](https://github.com/nogibjj/Leonard_Eshun_Individual_Project_Two/actions/workflows/rust_workflow.yml/badge.svg)](https://github.com/nogibjj/Leonard_Eshun_Individual_Project_Two/actions/workflows/rust_workflow.yml)  [![Python CI/CD](https://github.com/nogibjj/Leonard_Eshun_Individual_Project_Two/actions/workflows/python_workflow.yml/badge.svg)](https://github.com/nogibjj/Leonard_Eshun_Individual_Project_Two/actions/workflows/python_workflow.yml)


This repository is created as an assignment from the Data Engineering course, IDS 706. The aim is to create a rust project from an existing python project project best-practice elements. This is in fulfillment of Mini Project Eight and Individual Project 2.

**The requirements are:**
1. Have a functioning CI/CD for setup, lint, test (with a badge on the readme)
1. Rust source code: The code should comprehensively understand Rust's syntax and unique features.
1. Use of LLM: In your README, explain how you utilized an LLM in your coding process.
1. SQLite Database: Include a SQLite database and demonstrate CRUD (Create, Read, Update, Delete) operations.
1. Optimized Rust Binary: Include a process that generates an optimized Rust binary as a Gitlab Actions artifact that can be downloaded.
1. Highlight improvements in speed and resource usage by Rust relative to Python.
1. README.md: A file that clearly explains what the project does, its dependencies, how to run the program, and how Gitlab Copilot was used.
1. Github/Gitlab Actions: A workflow file that tests, builds, and lints your Rust code.
1. Video Demo: A YouTube link in README.md showing a clear, concise walkthrough and demonstration of your CLI binary.

## Table of Content
1. Use of LLM
1. Comparing speed between Rust and Python
1. Log of successful operations
1. The CLI Commands
1. Package Download
1. CLI Command Examples
1. The functions behind the CLI commands
1. A video describing the code
1. Project dependencies
1. References


## 1. Use of LLM
1. The Main LLM used was ChatGPT
1. I used it to convert python code snippets to rust
1. I used it to explain rust codes
1. I used it to get the meaning of errors    
I'll say ChatGPT was very effective in converting the code from Python to Rust. I had to make some changes but the LLM was very good, reducing the programming time tremendously.


## 2. Comparing speed between Rust and Python
The idea is to let the two languages do the same task with the least external influence on speed. For that reason, I won't use anything that involves network access, like data extraction. They'll both read the same amount of data from the same csv file and save the data to separate sqlite files residing in the same parent folder on the same storage and we see how long each language does it. A very similar code was used (Python code converted to Rust). All related information can be found in the `speed_test_data` folder.

The test functions were run three times:
![Test Ouput](assets/Speed_test_runs.png)

The result can be found [here](speed_test_data/Speed_Test_Result.md)

It appears from the result of this test that Rust is slightly faster than Python by 0.028997 seconds on the average.

## 3. Log of successful operations
`Cargo test` runs the tests in parallel by default and when cargo `test -- --test-threads 1` forces sequential run, the order is determined by the alphanumeric order of the test names. It's best practice to run unit tests in isolation, but I needed a particular sequence (extract, transform, load, Read, Create, Update, Delete), thus, I setup an integration test instead of unit tests, at `tests/integration_test.rs`. To control the sequence, I named the tests "test_ordernumber..." and used `test -- --test-threads 1 --nocapture`. This is to avoid a situation where it tries to test reading data when the data isn't loaded. The `--nocapture` option was used because by defualt Rust test programs hide the stdout of successful tests in order for the test output to be tidy and the side effect was that nothing was being written to my _Test_Log.md_ file. `--nocapture` prevents it from hiding the stdout. The test operation saved its steps to a log file to show the success of the operations. 

[Please find the file here](Test_Log.md)

The test output is shown below:
![Test Ouput](assets/Test_Output.png)


## 4. The CLI Commands
The standard form for a commnad in this scripts' CLI are:   
```
rust_sqlite_cli -alias "command" "arguments"
```


## 5. Package Download
Please download the compiled binary (`rust_sqlite_cli`) file here: [Binary for distribution](rust_sqlite_cli/target/release/rust_sqlite_cli)    
The CI/CD action results can be found under the Github _Actions_ page.   



**The commands are:**   
1. extract 
1. transform_n_load 
1. read_data 
1. read_all_data 
1. save_data 
1. delete_data 
1. update_data 
1. get_table_columns 

**Then follow with the relevant argument below, leaving a space between arguments:**   

```python
  extract:   
	url			(type = string)
        file_name		(type = string)

	transform_n_load:   
        local_dataset		(type = string)
        database_name		(type = string)
        new_data_tables 	(type = dict)
        new_lookup_tables 	(type = dict)
        column_attributes 	(type = dict)
        column_map 		(type = dict)

    read_data:   
        database_name		(type = string)
        table_name		(type = string)
        data_id 		(type = int)

    read_all_data:   
        database_name		(type = string)
        table_name		(type = string)

    save_data:   
        database_name		(type = string)
        table_name		(type = string)
        row 			(type = list)

    update_data:   
        database_name		(type = string)
        table_name		(type = string)
        data_id			(type = int)
        things_to_update 	(type = dict)

    delete_data:   
        database_name		(type = string)
        table_name		(type = string)
        data_id 		(type = int)

    get_table_columns:   
        database_name		(type = string)
        table_name		(type = string)
```

> [!IMPORTANT]
> It's important to provide the arguments in the order and formats as desribed above for the CLI to work.   
>When adding arguments like lists or dictionary, please ensure that the outer quotes are double quotes and the individual inner items have single quotes (or vice-versa) so it knows where the list starts and ends.


## 6. CLI Command Examples

1\. extract:   
```python
rust_sqlite_cli -e "https://data.cityofnewyork.us/resource/c3uy-2p5r.csv" "air_quality.csv"
```

2\. transform_and_load:   
```python
rust_sqlite_cli -l "air_quality.csv" "air_quality.db" '{"air_quality":["air_quality_id","fn_indicator_id","fn_geo_id","time_period","start_date","data_value"]}' '{"indicator":["indicator_id","indicator_name","measure","measure_info"],"geo_data":["geo_id","geo_place_name","geo_type_name"]}' '{"air_quality_id":"INTEGER PRIMARY KEY","indicator_id":"INTEGER PRIMARY KEY","indicator_name":"TEXT","measure":"TEXT","measure_info":"TEXT","geo_type_name":"TEXT","geo_id":"INTEGER PRIMARY KEY","geo_place_name":"TEXT","time_period":"TEXT","start_date":"TEXT","data_value":"REAL","fn_indicator_id":"INTEGER","fn_geo_id":"INTEGER"}' '{"air_quality_id":0,"indicator_id":1,"indicator_name":2,"measure":3,"measure_info":4,"geo_type_name":5,"geo_id":6,"geo_place_name":7,"time_period":8,"start_date":9,"data_value":10,"fn_geo_id":6,"fn_indicator_id":1}'
```

3\. read_data:   
```python
rust_sqlite_cli -q "air_quality.db" geo_data 101
```

4\. read_all_data:   
```python
rust_sqlite_cli -a "air_quality.db" indicator
```

5\. save_data:    
```python
rust_sqlite_cli -s "air_quality.db" geo_data "['100000', 'Lancaster', 'UFO']"
```

6\. delete_data:    
```python
rust_sqlite_cli -d "air_quality.db" geo_data '100000'
```

7\. update_data:    
```python
rust_sqlite_cli -u "air_quality.db" "geo_data" '{"geo_place_name": "Northeast-Bronx"}' '102' 
```

8\. get_table_columns:    
```python
sqlite_etl -c "air_quality.db" "air_quality"
```

## 7. The functions behind the CLI commands

1. **extract** to extract the read an external csv file via its url and save to file in the /data folder using the name you give it. The database will be created if it doesn't exist.
	```python
	extract(url: str, file_name: str,) -> str
	```
	The parameters are:
	- url : The url for the external CSV file
	- file_name : The file name to use to save the CSV file locally

	>**Note:**
	>Give the CSV file a header (first row).


1. **transform_n_load** to create a number of tables in the SQLite database based on the table structures you give it for transformation, then saves the content of the csv file to the tables you created. 
	```python
	transform_n_load(    
		local_dataset: str,
    	database_name: str,
    	new_data_tables: dict,
    	new_lookup_tables: dict,
    	column_attributes: dict,
    	column_map: dict,)
	```
	The parameters are:
	- local_dataset : The local CSV file to load
	- database_name : The name of the database to be created	
	- new_data_tables : A dictionary of the tables non-lookup tables to be created. The key is the table name and the value is an array of columns.
	- new_lookup_tables : A dictionary of the tables lookup tables to be created. The key is the table name and the value is an array of columns.
	- column_attributes : A dictionary of the column attributes, eg. Integer, Primary Key. The key is the column name and the values are the attributes.
	- column_map : A dictionary maping the columns in the new tables defined above to the column indices in the CSV file. The key is the column.

	>**Note:**
	>The ID Primary Key of the table should always be the first column. 
	>Column names also shouldn't have spaces.


1. **read_data** to readon one data from the SQLite database based on the record id you give it.
	```python
	read_data(database_name: str, table_name: str, data_id: int)
	```
	The parameters are:
	- database_name : The name of the SQLite database containing the data.
	- table_name : The name of the table in the SQLite database.	
	- data_id : The ID of the record to be read.	

1. **read_all_data** to read all the records from the SQLite database.
	```python
	read_all_data(database_name: str, table_name: str)
	```
	The parameters are:
	- database_name : The name of the SQLite database containing the data.
	- table_name : The name of the table in the SQLite database.	


1. **save_data** to save records to a table you give it, following the table column structure.
	```python
	save_data(database_name: str, table_name: str, row: list)
	```
	The parameters are:
	- database_name : The name of the SQLite database containing the data.
	- table_name : The name of the table in the SQLite database.
	- row : A list of the items to be saved. The order should follow the exact output of the ```get_table_columns``` function for that table.


1. **delete_data** to delete a record from the database given a record ID.
	```python
	delete_data(database_name: str, table_name: str, data_id: int)
	```
	The parameters are:
	- database_name : The name of the SQLite database containing the data.
	- table_name : The name of the table in the SQLite database.	
	- data_id : The ID of the record to be deleted.	

1. **update_data** to update a record in the database using the table columns and a record ID.
	```python
	update_data(database_name: str, table_name: str, things_to_update: dict, data_id: int)
	```
	The parameters are:
	- database_name : The name of the SQLite database containing the data.
	- table_name : The name of the table in the SQLite database.
	- things_to_update : A dictionary of the items to be updated. The key is the column name and the value is the new data. The column names use must be in the output of the  ```get_table_columns``` function for that table.
	- data_id : The ID of the record to be updated.	

1. **get_table_columns** to get the column names of a table. This is useful for saving and updating.
	```python
	get_table_columns(database_name: str, table_name: str)
	```
	The parameters are:
	- database_name : The name of the SQLite database.
	- table_name : The name of the table in the SQLite database.   


## 8. A video describing the code can be found here
[![Project Code Description](https://markdown-videos-api.jorgenkh.no/url?url=https%3A%2F%2Fwww.youtube.com%2Fwatch%3Fv%3DV8W3qpIdc_k)](https://www.youtube.com/watch?v=V8W3qpIdc_k)


## 9. Project dependencies  
**Rust**
1. reqwest (version 0.11)
1. clap (version 4.5.20)
1. csv (1.3.0)
1. rusqlite (0.32.1)
1. serde (1.0.214)
1. serde_json (1.0.132)
1. maplit (1.0.2)
1. chrono (0.4)

**Python**
1. black (version 22.3.0)
1. pytest (version 7.4.0)
1. ruff (version 0.6.5)


## 10. References  
\- [Integration Testing](https://doc.rust-lang.org/rust-by-example/testing/integration_testing.html)   
\- [Sequential Testing](https://users.rust-lang.org/t/run-tests-sequentially/16397)   
\- [Capture of stdout](https://stackoverflow.com/questions/25106554/why-doesnt-println-work-in-rust-unit-tests)   

