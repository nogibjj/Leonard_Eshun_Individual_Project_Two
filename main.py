import json


def add(a, b):
    return a + b


def get_the_capital_of_a_country(country: str) -> str:
    capital = ""
    # Open and read the JSON file
    with open(file="country_capital.json", mode="r", encoding="utf-8") as file:
        data = json.load(file)
        capital = (
            data[country.lower()]
            if data.get(country.lower(), "") != ""
            else "The country you specified was not found!"
        )
    return capital


if __name__ == "__main__":
    print(add(1, 2))
    print(get_the_capital_of_a_country("Ghana"))
