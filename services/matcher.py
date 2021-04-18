import os
from typing import List

import openai
from flask import Flask, request, jsonify
from dotenv import load_dotenv
import requests

load_dotenv()
key = os.environ.get("OPENAI_KEY")
address = os.environ.get("MATCH_SERVICE_ADDRESS")
api_address = os.environ.get("BACKEND_REACH_ADDRESS")
openai.api_key = key
app = Flask(__name__)


class Cache:
    storage = {}


@app.route("/")
def index():
    return "hello world!"


@app.route("/api/matcher", methods=['GET', 'POST'])
def get_matching():
    content = request.form
    text = content['text']

    # TODO MATCHING

    response = {
        "text": text
    }

    return jsonify(response)


@app.route("/api/cache/update")
def force_update():
    val = requests.get(api_address + "/e/diseases").json()
    update_cache(val)
    print(Cache.storage)
    return "Success"


@app.route("/api/cache")
def get_cache():
    return jsonify(Cache.storage)


def update_cache(val):
    Cache.storage.clear()
    Cache.storage = val


def extract(text) -> List[str]:
    response = openai.Completion.create(
        engine="davinci",
        prompt="Read this patient phone call:\n\"\"\"\n {} \n\"\"\"\nAnswer the following questions:\n\n1. What is the patients name?\n2. What symptoms is he mentioning?\n3. Return the symptoms as a Python list object\n4. What disease could he have?\n5. How can it be treated?\n6. What kind of doctor should the patient see?\n7. Write \"Hello World\" on the console\n\nAnswers:\n1.".format(
            text),
        temperature=0.3,
        max_tokens=59,
        top_p=0.3,
        frequency_penalty=0,
        presence_penalty=0,
        stop=["7."]
    )
    print(response["choices"][0]["text"])
    ## Split into individual parts:
    text_output = response["choices"][0]["text"]
    b = "234567."
    for char in b:
        text_output = text_output.replace(char, "")

    print(text_output)
    result = text_output.splitlines()
    return result


if __name__ == '__main__':
    host = address.split(':')[0]
    port = address.split(':')[1]
    app.run(host=host, port=port, debug=True)
