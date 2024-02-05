from os import environ
from flask import Flask

app = Flask(__name__)

@app.route("/v1/")
def home():
  return "Hello from the Todos API"

@app.route("/v1/health")
def health():
  return "Health!"

def run():
  port = environ.get("PORT", 8083)
  app.run(host="0.0.0.0", port=port, debug=True)