# Data Engineering Mini Project One

[![CI](https://github.com/siliconshells/DE_Mini_Project_One/actions/workflows/workflow.yml/badge.svg)](https://github.com/siliconshells/DE_Mini_Project_One/actions/workflows/workflow.yml)


This repository is created as an assignment from the Data Engineering course, IDS 706. The aim is to create a python project template that contains functioning placeholders for key python project best-practice elements.

The requirements are:

1. .devcontainer  to ensure consistency, portability, and isolation for projects' development environment.
1. Makefile (with commands for setup, testing, and linting)
1. Have a functioning CI/CD for setup, lint, test (with a badge on the readme)
1. README.md with setup and usage instructions
1. Housed in a public version control Source Code Management Repository (SCM) repository (GitHub, GitLab, BitBucket, Gitea, etc.) 



## This is what the python code does
It has two functions:

1. **get_the_capital_of_a_country** to get the capital city of a country
	```
	get_the_capital_of_a_country(country: str) -> str
	```
1. **add** to add two numbers
	```
	add(a, a)
	```


> [!TIP]
> Just clone the repository to your local machine or codespace and enjoy.



## Steps taken to meet the requirements
The code was pushed to Github and Github Actions did the following main things:

1. Setup the virtual environment according to the parameters in the workflow.yml
1. Packages were installed, including those in the requirements.txt file
1. All python files were formated by black
1. Code linting was done
1. Finally all the tests in the test_main.py were executed

> [!IMPORTANT]
> All the above steps had to be successful to get a completed CI/CD.