import random
import os
from dotenv import load_dotenv
import requests
import names
import json

load_dotenv()
api_address = os.environ.get("BACKEND_REACH_ADDRESS")

response = requests.get(api_address + "/departments")
content = response.json()

departments = []

for dep in content:
    departments.append({"id": dep["id"], "name": dep["name"]})

for dep in departments:
    for i in range(0, random.randrange(2, 5)):
        name = names.get_full_name()
        request = {
            "name": name,
            "occupied": False,
            "department": int(dep['id'])
        }
        # val = json.dumps(request)
        req = requests.post(api_address + "/doctors", json=request)
        print(req.status_code)
