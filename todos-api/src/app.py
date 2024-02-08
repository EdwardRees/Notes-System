from os import environ

import uvicorn
from fastapi import FastAPI 

app = FastAPI()

@app.get("/v1")
def home():
    return "Hello from the Todos API"

@app.get("/v1/health")
def health():
    return "Healthy!"


def run():
    port = int(environ.get("PORT", 8085))
    uvicorn.run("app:app", port=port, reload=True, access_log=False)
