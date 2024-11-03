import os.path

def log_tests(log, issql=False, header=False, last_in_group=False, new_log_file=False, file_name = None):
    log = log.strip()
    if file_name is None:
        file_name = db_path + "python_code/Test_Log.md"
    else:
        file_name = db_path + file_name
    
    with open(file_name, "w" if new_log_file else "a") as file:
        if issql:
            file.write(f"\n```sql\n{log}\n```\n\n")
        elif header:
            file.write(f"### {log} ### \n")
        elif last_in_group:
            file.write(f"{log}\n\n\n")
        else:
            file.write(f"{log} <br />")



BASE_DIR = os.path.dirname(os.path.abspath(__file__))
db_path = os.path.join(BASE_DIR, "../../")
