import sqlalchemy as db
from os import environ

def get_db_connection():
  db_url = environ.get("DATABASE_URL", None)
  if(db_url is None):
    print("Error: Missing Database URL!")
    return ""
  engine = db.create_engine(db_url)
  connection = engine.connect()
  return connection
