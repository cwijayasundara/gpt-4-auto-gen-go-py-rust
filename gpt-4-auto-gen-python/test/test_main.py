from fastapi.testclient import TestClient
from main import app

client = TestClient(app)


def test_create_trade():
    response = client.post(
        "/trades/",
        json={
            "value": 1000.0,
            "date": "2023-01-01",
            "trader": "John"
        },
    )
    assert response.status_code == 200
    assert response.json()["value"] == 1000.0
    assert response.json()["date"] == "2023-01-01"
    assert response.json()["trader"] == "John"

# Add more tests for other CRUD operations
