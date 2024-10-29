from main import add
from main import get_the_capital_of_a_country


def test_add():
    """Tewting the add function"""
    assert add(2, 2) == 4
    assert add(3, 2) == 5


def test_countries():
    """testing out get_the_capital_of_a_country function"""
    assert get_the_capital_of_a_country("United States") == "Washington D.C."
    assert get_the_capital_of_a_country("Ghana") == "Accra"
    assert get_the_capital_of_a_country("united states") == "Washington D.C."
    assert (
        get_the_capital_of_a_country("London")
        == "The country you specified was not found!"
    )


if __name__ == "__main__":
    test_add()
    test_countries()
    print("Test completed successfully")
