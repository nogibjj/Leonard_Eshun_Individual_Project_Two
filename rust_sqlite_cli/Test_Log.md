### Extraction Test ### 

Removing existing CSV file if it exists <br />
Confirming that CSV file doesn't exist... <br />
Test Successful <br />
Extracting data and saving... <br />
Testing if CSV file exists... <br />
Extraction Test Successful


### Transform and Load Test ### 

Removing existing sqlite file if it exists <br />
Confirming that sqlite file doesn't exist... <br />
Creating non-lookup table: air_quality <br />
Creating lookup table: geo_data <br />
Creating lookup table: indicator <br />
Tables created. <br />
Tables created. <br />
Tables created. <br />
Inserting table data completed <br />
Testing if sqlite file exists... <br />
Transform and Load Test Successful


### One Record Reading Test ### 

Executing query... <br />
Asserting that row[0][data_value] == 16.4 <br />
Assert Successful <br />
One Record Reading Test Successful


### All Records Reading Test ### 

Asserting that len(rows) == 18016 <br />
All Records Reading Test Successful


### Record Saving Test ### 

Asserting there's no record in geo_data with ID 100000 <br />
Executing query... <br />
Assert Successful <br />
Saving new record with ID 100000 <br />
Executing query... <br />
Executing query... <br />
Asserting there's now a record in geo_data with ID 100000 <br />
Executing query... <br />
Assert Successful <br />
Record Saving Test Successful


### Record Update Test ### 

Asserting 'geo_place_name' in geo_data for row ID 100000 is 'Lancaster' <br />
Executing query... <br />
Assert Successful <br />
Updating 'geo_place_name' in geo_data for row ID 100000 to 'Duke' <br />
Executing query... <br />
Asserting 'geo_place_name' in geo_data for row ID 100000 is now 'Duke' <br />
Executing query... <br />
Assert Successful <br />
Record Update Test Successful


### Record Deletion Test ### 

Asserting there's a record in geo_data for row ID 100000 <br />
Executing query... <br />
Assert Successful <br />
Deleting record with ID 100000 <br />
Executing query... <br />
Asserting there's no record in geo_data with ID 100000 <br />
Executing query... <br />
Assert Successful <br />
Record Deletion Test Successful


### Reading All Column Test ### 

Executing query... <br />
Asserting the air_quality table has six (6) columns <br />
Assert Successful <br />
Reading All Column Test Successful


